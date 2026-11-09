use clang_ast::Kind;
use codegen::rustfmt;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

const BUILDDIR: &str = "mesa/builddir";
const DEVICE_INFO: &str = "../mesa/src/intel/dev/intel_device_info.c";
const ISL: &str = "../mesa/src/intel/isl/isl.c";
const IRIS_MIN_VER: i128 = 8;

type Node = clang_ast::Node<Clang>;

#[derive(Deserialize)]
struct Named {
    name: Option<String>,
}

#[derive(Deserialize)]
struct Operator {
    opcode: String,
}

#[derive(Deserialize)]
struct Literal {
    value: String,
}

#[derive(Deserialize)]
struct Constant {
    value: Option<String>,
}

#[derive(Deserialize)]
struct Referenced {
    kind: Kind,
    name: Option<String>,
}

#[derive(Deserialize)]
struct DeclRef {
    #[serde(rename = "referencedDecl")]
    referenced_decl: Referenced,
}

#[derive(Deserialize)]
enum Clang {
    RecordDecl(Named),
    FieldDecl(Named),
    VarDecl(Named),
    FunctionDecl(Named),
    EnumDecl(Named),
    EnumConstantDecl(Named),
    CompoundStmt,
    SwitchStmt,
    CaseStmt,
    DefaultStmt,
    ReturnStmt,
    BinaryOperator(Operator),
    IntegerLiteral(Literal),
    ConstantExpr(Constant),
    ImplicitCastExpr,
    ParenExpr,
    CStyleCastExpr,
    CompoundLiteralExpr,
    InitListExpr,
    ImplicitValueInitExpr,
    DeclRefExpr(DeclRef),
    Other,
}

#[derive(Deserialize)]
struct CompileCommand {
    directory: String,
    command: String,
    file: String,
}

enum Eval<'a> {
    Integer(i128),
    Constant(&'a str),
    Variable(&'a str),
    List(&'a Node),
}

struct Compiler {
    directory: PathBuf,
    arguments: Vec<String>,
}

struct PatEntry {
    index: i128,
}

struct Pat {
    scanout: PatEntry,
    writecombining: PatEntry,
    compressed: PatEntry,
    compressed_scanout: PatEntry,
}

struct Info {
    ver: i128,
    verx10: i128,
    has_llc: bool,
    has_local_mem: bool,
    has_aux_map: bool,
    has_flat_ccs: bool,
    gt: i128,
    platform: String,
    pat: Pat,
}

struct Chipset<'a> {
    pci_id: i128,
    family: &'a str,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Decl {
    Record,
    Variable,
    Function,
    Enumeration,
}

struct Index<'a> {
    decls: HashMap<(Decl, &'a str), &'a Node>,
    fields: HashMap<(&'a str, &'a str), usize>,
    switches: HashMap<&'a str, &'a Node>,
}

fn compile_commands(root: &Path) -> HashMap<String, CompileCommand> {
    let path = root.join(BUILDDIR).join("compile_commands.json");
    let text = std::fs::read_to_string(path).unwrap();
    let commands: Vec<CompileCommand> = serde_json::from_str(&text).unwrap();
    let mut by_file = HashMap::with_capacity(commands.len());
    for command in commands {
        by_file.entry(command.file.clone()).or_insert(command);
    }
    by_file
}

impl Compiler {
    fn new(commands: &HashMap<String, CompileCommand>, source: &str) -> Self {
        let command = &commands[source];
        let words = shlex::split(&command.command).unwrap();
        let compiler = match words[0].ends_with("ccache") {
            true => 1,
            false => 0,
        };
        Self {
            directory: PathBuf::from(&command.directory),
            arguments: words[compiler + 1..].to_vec(),
        }
    }

    fn dump(&self, filter: &str) -> Vec<Node> {
        let output = Command::new("clang")
            .current_dir(&self.directory)
            .args(&self.arguments)
            .args([
                "-Wno-everything",
                "-fsyntax-only",
                "-Xclang",
                "-ast-dump=json",
            ])
            .args(["-Xclang", &format!("-ast-dump-filter={filter}")])
            .stderr(Stdio::inherit())
            .output()
            .unwrap();
        assert!(output.status.success(), "clang failed");
        serde_json::Deserializer::from_slice(&output.stdout)
            .into_iter::<Node>()
            .map(Result::unwrap)
            .collect()
    }
}

fn parse(value: &str) -> i128 {
    value.parse().unwrap()
}

fn eval(node: &Node) -> Eval<'_> {
    match &node.kind {
        Clang::IntegerLiteral(literal) => Eval::Integer(parse(&literal.value)),
        Clang::ImplicitValueInitExpr => Eval::Integer(0),
        Clang::InitListExpr => Eval::List(node),
        Clang::ConstantExpr(constant) => match &constant.value {
            Some(value) => Eval::Integer(parse(value)),
            None => eval(&node.inner[0]),
        },
        Clang::ImplicitCastExpr | Clang::ParenExpr | Clang::CStyleCastExpr => eval(&node.inner[0]),
        Clang::DeclRefExpr(reference) => {
            let decl = &reference.referenced_decl;
            let name = decl.name.as_deref().unwrap();
            match decl.kind {
                Kind::EnumConstantDecl => Eval::Constant(name),
                Kind::VarDecl => Eval::Variable(name),
                kind => panic!("reference to {kind:?}"),
            }
        }
        _ => panic!("cannot evaluate node {:?}", node.id),
    }
}

fn named(node: &Node) -> Option<&str> {
    match &node.kind {
        Clang::RecordDecl(decl)
        | Clang::FieldDecl(decl)
        | Clang::VarDecl(decl)
        | Clang::FunctionDecl(decl)
        | Clang::EnumDecl(decl)
        | Clang::EnumConstantDecl(decl) => decl.name.as_deref(),
        _ => None,
    }
}

fn decl(kind: &Clang) -> Option<Decl> {
    match kind {
        Clang::RecordDecl(_) => Some(Decl::Record),
        Clang::VarDecl(_) => Some(Decl::Variable),
        Clang::FunctionDecl(_) => Some(Decl::Function),
        Clang::EnumDecl(_) => Some(Decl::Enumeration),
        _ => None,
    }
}

impl<'a> Index<'a> {
    fn new(nodes: &'a [Node]) -> Self {
        let mut index = Index {
            decls: HashMap::new(),
            fields: HashMap::new(),
            switches: HashMap::new(),
        };
        for node in nodes {
            let (Some(kind), Some(name)) = (decl(&node.kind), named(node)) else {
                continue;
            };
            if node.inner.is_empty() || index.decls.contains_key(&(kind, name)) {
                continue;
            }
            index.decls.insert((kind, name), node);
            match kind {
                Decl::Record => {
                    let fields = node
                        .inner
                        .iter()
                        .filter(|inner| matches!(inner.kind, Clang::FieldDecl(_)));
                    for (position, field) in fields.enumerate() {
                        if let Some(field) = named(field) {
                            index.fields.entry((name, field)).or_insert(position);
                        }
                    }
                }
                Decl::Function => {
                    let body = node
                        .inner
                        .iter()
                        .filter(|inner| matches!(inner.kind, Clang::CompoundStmt));
                    let switch = body
                        .flat_map(|body| &body.inner)
                        .find(|inner| matches!(inner.kind, Clang::SwitchStmt));
                    if let Some(switch) = switch {
                        index.switches.insert(name, switch);
                    }
                }
                Decl::Variable | Decl::Enumeration => {}
            }
        }
        index
    }

    fn get(&self, kind: Decl, name: &str) -> &'a Node {
        self.decls
            .get(&(kind, name))
            .unwrap_or_else(|| panic!("no definition of {name}"))
    }

    fn switch(&self, function: &str) -> &'a Node {
        self.switches
            .get(function)
            .unwrap_or_else(|| panic!("no switch in {function}"))
    }

    fn field<'b>(&self, record_name: &str, list: &'b Node, name: &str) -> &'b Node {
        if matches!(list.kind, Clang::ImplicitValueInitExpr) {
            return list;
        }
        let position = self
            .fields
            .get(&(record_name, name))
            .unwrap_or_else(|| panic!("no field {name} in {record_name}"));
        &list.inner[*position]
    }

    fn integer_field(&self, record_name: &str, list: &Node, name: &str) -> i128 {
        match eval(self.field(record_name, list, name)) {
            Eval::Integer(value) => value,
            _ => panic!("{record_name}.{name} is not an integer"),
        }
    }

    fn list_field<'b>(&self, record_name: &str, list: &'b Node, name: &str) -> &'b Node {
        let node = self.field(record_name, list, name);
        if matches!(node.kind, Clang::ImplicitValueInitExpr) {
            return node;
        }
        match eval(node) {
            Eval::List(list) => list,
            _ => panic!("{record_name}.{name} is not an initializer list"),
        }
    }
}

fn enum_values(index: &Index<'_>, name: &str) -> HashMap<String, i128> {
    let mut values = HashMap::new();
    let mut next = 0;
    for constant in &index.get(Decl::Enumeration, name).inner {
        let Clang::EnumConstantDecl(decl) = &constant.kind else {
            continue;
        };
        let constant_name = decl.name.clone().unwrap();
        let value = match constant.inner.first() {
            None => next,
            Some(init) => match eval(init) {
                Eval::Integer(value) => value,
                Eval::Constant(other) => values[other],
                _ => panic!("the initializer of {constant_name} in {name} is not a constant"),
            },
        };
        values.insert(constant_name, value);
        next = value + 1;
    }
    values
}

fn device(index: &Index<'_>, platforms: &HashMap<String, i128>, name: &str) -> Info {
    let record_name = "intel_device_info";
    let var = index.get(Decl::Variable, name);
    let Eval::List(list) = eval(&var.inner[0]) else {
        panic!("{name} has no initializer list");
    };
    let Eval::Constant(platform) = eval(index.field(record_name, list, "platform")) else {
        panic!("{name}.platform is not an enum constant");
    };
    assert!(
        platforms.contains_key(platform),
        "{name}.platform is {platform}, which is not in intel_platform"
    );
    let pat = index.list_field(record_name, list, "pat");
    let entry = |name: &str| {
        let record_name = "intel_device_info_pat_entry";
        let entry = index.list_field("intel_device_info_pat_desc", pat, name);
        PatEntry {
            index: index.integer_field(record_name, entry, "index"),
        }
    };
    let ver = index.integer_field(record_name, list, "ver");
    let verx10 = match index.integer_field(record_name, list, "verx10") {
        0 => ver * 10,
        verx10 => verx10,
    };
    Info {
        ver,
        verx10,
        has_llc: index.integer_field(record_name, list, "has_llc") != 0,
        has_local_mem: index.integer_field(record_name, list, "has_local_mem") != 0,
        has_aux_map: index.integer_field(record_name, list, "has_aux_map") != 0,
        has_flat_ccs: index.integer_field(record_name, list, "has_flat_ccs") != 0,
        gt: index.integer_field(record_name, list, "gt"),
        platform: platform.to_string(),
        pat: Pat {
            scanout: entry("scanout"),
            writecombining: entry("writecombining"),
            compressed: entry("compressed"),
            compressed_scanout: entry("compressed_scanout"),
        },
    }
}

fn assigned<'a>(statement: &'a Node, target: &str) -> Option<&'a Node> {
    let Clang::BinaryOperator(operator) = &statement.kind else {
        return None;
    };
    if operator.opcode != "=" {
        return None;
    }
    let mut lhs = &statement.inner[0];
    loop {
        if let Clang::DeclRefExpr(reference) = &lhs.kind {
            let name = reference.referenced_decl.name.as_deref()?;
            return (name == target).then(|| &statement.inner[1]);
        }
        lhs = &lhs.inner[0];
    }
}

fn chipset_table<'a>(index: &Index<'a>) -> Vec<Chipset<'a>> {
    let function = "intel_device_info_init_common";
    let switch = index.switch(function);
    let cases = switch.inner.last().unwrap();
    let mut out = Vec::new();
    let mut pending = None;
    for statement in &cases.inner {
        let statement = match statement.kind {
            Clang::CaseStmt => {
                let Eval::Integer(pci_id) = eval(&statement.inner[0]) else {
                    panic!("{function} has a case value that is not an integer");
                };
                if let Some(previous) = pending.replace(pci_id) {
                    panic!("case {previous:#x} of {function} assigns no device info");
                }
                &statement.inner[1]
            }
            Clang::DefaultStmt => break,
            _ => statement,
        };
        if let Some(value) = assigned(statement, "devinfo") {
            let pci_id = pending
                .take()
                .unwrap_or_else(|| panic!("{function} assigns device info outside of a case"));
            let Eval::Variable(family) = eval(value) else {
                panic!("case {pci_id:#x} of {function} does not assign a variable");
            };
            out.push(Chipset { pci_id, family });
        }
    }
    if let Some(pci_id) = pending {
        panic!("case {pci_id:#x} of {function} assigns no device info");
    }
    out
}

fn switch_returns(index: &Index<'_>, name: &str) -> HashMap<i128, i128> {
    let switch = index.switch(name);
    let mut out = HashMap::new();
    for case in &switch.inner.last().unwrap().inner {
        if !matches!(case.kind, Clang::CaseStmt) {
            continue;
        }
        let Eval::Integer(key) = eval(&case.inner[0]) else {
            panic!("{name} case value");
        };
        let statement = &case.inner[1];
        assert!(matches!(statement.kind, Clang::ReturnStmt));
        let Eval::Integer(value) = eval(&statement.inner[0]) else {
            panic!("{name} return value");
        };
        out.insert(key, value);
    }
    out
}

fn in_range(platforms: &HashMap<String, i128>, platform: &str, range: &str) -> bool {
    let value = platforms[platform];
    platforms[&format!("INTEL_PLATFORM_{range}_START")] <= value
        && value <= platforms[&format!("INTEL_PLATFORM_{range}_END")]
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let commands = compile_commands(root);
    let compiler = Compiler::new(&commands, DEVICE_INFO);
    let nodes = compiler.dump("intel_");
    let index = Index::new(&nodes);
    let platforms = enum_values(&index, "intel_platform");
    let mut chipsets = chipset_table(&index);
    chipsets.sort_by_key(|chipset| chipset.pci_id);
    let isl = Compiler::new(&commands, ISL);
    let clear_color_nodes = isl.dump("CLEAR_COLOR_length");
    let clear_color_lengths = switch_returns(&Index::new(&clear_color_nodes), "CLEAR_COLOR_length");
    let mut out = String::from(
        "use crate::drivers::intel::device_info::DeviceInfo;\n\
         use crate::drivers::intel::device_info::Pat;\n\
         use crate::drivers::intel::device_info::PatEntry;\n\n",
    );
    writeln!(
        out,
        "pub static DEVICES: [DeviceInfo; {}] = [",
        chipsets.len()
    )
    .unwrap();
    for chipset in &chipsets {
        let info = device(&index, &platforms, chipset.family);
        let mtl_or_arl = in_range(&platforms, &info.platform, "MTL")
            || in_range(&platforms, &info.platform, "ARL");
        let dg2 = in_range(&platforms, &info.platform, "DG2")
            || in_range(&platforms, &info.platform, "ATSM");
        let skl = info.platform == "INTEL_PLATFORM_SKL";
        let clear_color_state_size = match clear_color_lengths.get(&info.verx10) {
            Some(length) => length * 4,
            None => {
                assert!(
                    info.ver < IRIS_MIN_VER,
                    "no CLEAR_COLOR length for {}",
                    chipset.family
                );
                0
            }
        };
        let pat = &info.pat;
        let entry = |entry: &PatEntry| format!("PatEntry {{ index: {} }}", entry.index);
        writeln!(
            out,
            "DeviceInfo {{ pci_id: {:#06x}, ver: {}, verx10: {}, dg2: {dg2}, \
             mtl_or_arl: {mtl_or_arl}, skl: {skl}, gt: {}, has_llc: {}, has_local_mem: {}, \
             has_aux_map: {}, has_flat_ccs: {}, clear_color_state_size: {clear_color_state_size}, \
             pat: Pat {{ scanout: {}, \
             writecombining: {}, compressed: {}, compressed_scanout: {} }} }},",
            chipset.pci_id,
            info.ver,
            info.verx10,
            info.gt,
            info.has_llc,
            info.has_local_mem,
            info.has_aux_map,
            info.has_flat_ccs,
            entry(&pat.scanout),
            entry(&pat.writecombining),
            entry(&pat.compressed),
            entry(&pat.compressed_scanout),
        )
        .unwrap();
    }
    out.push_str("];\n");
    let formatted = rustfmt(root, &out);
    std::fs::write(
        root.join("dmabuf-allocator-impl/src/drivers/intel/devices.rs"),
        formatted,
    )
    .unwrap();
}

use super::*;
use vulkan_testgen::distribute::Job as _;

fn size(width: u32, height: u32) -> Size {
    Size { width, height }
}

#[test]
fn every_job_is_decoded_as_it_was_encoded() {
    let mut jobs = vec![
        Job::Implicit {
            node: 70,
            format: 147,
        },
        Job::Simple {
            driver: 13,
            node: 200,
            format: 3,
        },
        Job::SimpleRank {
            driver: 13,
            node: 200,
            format: 3,
        },
    ];
    for group in Group::ALL {
        jobs.push(Job::Describe { group, node: 9 });
        jobs.push(Job::GroupRank {
            group,
            node: 31,
            format: 5,
        });
        jobs.push(Job::Coverage {
            group,
            pair: 4000,
            node: 12,
        });
    }
    for backend in Backend::ALL {
        jobs.push(Job::Evaluate {
            backend,
            pair: 77,
            size: size(16384, 1),
        });
        jobs.push(Job::Record {
            backend,
            pair: u32::MAX,
            size: size(1, 16384),
        });
    }
    for job in jobs {
        assert_eq!(Job::decode(job.encode()), job);
    }
}

#[test]
#[should_panic(expected = "invalid job kind 8")]
fn an_unknown_job_kind_is_refused() {
    Job::decode([8, 0, 0, 0, 0, 0]);
}

#[test]
#[should_panic(expected = "invalid backend 6")]
fn an_unknown_backend_is_refused() {
    Job::decode([EVALUATE, 6, 0, 1, 1, 0]);
}

#[test]
fn every_group_has_both_backends() {
    for group in Group::ALL {
        for api in [Api::Vulkan, Api::Gbm] {
            assert!(Backend::ALL.contains(&Backend { group, api }));
        }
        assert_eq!(Group::from_name(group.name()), Some(group));
    }
    assert_eq!(Group::from_name("dumb"), None);
}

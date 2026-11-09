use super::*;
use std::mem::offset_of;

#[test]
fn export_sync_file_matches_the_kernel_header() {
    assert_eq!(size_of::<dma_buf_export_sync_file>(), 8);
    assert_eq!(offset_of!(dma_buf_export_sync_file, flags), 0);
    assert_eq!(offset_of!(dma_buf_export_sync_file, fd), 4);
    assert_eq!(DMA_BUF_IOCTL_EXPORT_SYNC_FILE, 0xc008_6202);
}

#[test]
fn import_sync_file_matches_the_kernel_header() {
    assert_eq!(size_of::<dma_buf_import_sync_file>(), 8);
    assert_eq!(offset_of!(dma_buf_import_sync_file, flags), 0);
    assert_eq!(offset_of!(dma_buf_import_sync_file, fd), 4);
    assert_eq!(DMA_BUF_IOCTL_IMPORT_SYNC_FILE, 0x4008_6203);
}

#[test]
fn sync_write_matches_the_kernel_header() {
    assert_eq!(DMA_BUF_SYNC_WRITE, 2);
}

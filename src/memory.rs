use x86_64::{structures::paging::PageTable, VirtAddr};

/// This function is unsafe since the caller must guarantee that the
/// physical_memory_offset is valid.
///
/// The function should only be called once because it is returning a
/// `&mut` and those can not be aliased (UB)
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr)
    -> &'static mut PageTable 
{
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

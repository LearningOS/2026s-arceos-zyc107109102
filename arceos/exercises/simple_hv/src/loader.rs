use std::io::{self, Read};
use std::fs::File;
use axhal::paging::MappingFlags;
use axhal::mem::{PAGE_SIZE_4K, phys_to_virt};
use axmm::AddrSpace;
use crate::VM_ENTRY;

pub fn load_vm_image(fname: &str, uspace: &mut AddrSpace) -> io::Result<()> {
    //把缓冲区大小改成文件大小
    let mut file = File::open(fname)?;
    let file_size = file.metadata()?.len() as usize;
    let mut buf = alloc::vec![0u8; file_size];
    file.read_exact(&mut buf)?;
    //按页对齐映射
    let num_pages = (file_size + PAGE_SIZE_4K - 1) / PAGE_SIZE_4K;
    for i in 0..num_pages {
        uspace.map_alloc(
            (VM_ENTRY + i * PAGE_SIZE_4K).into(),
            PAGE_SIZE_4K,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE | MappingFlags::USER,
            true,
        ).unwrap();
    }
    let (paddr, _, _) = uspace
        .page_table()
        .query(VM_ENTRY.into())
        .unwrap_or_else(|_| panic!("Mapping failed for segment: {:#x}", VM_ENTRY));
    //拷贝
    ax_println!("paddr: {:#x}", paddr);
    unsafe {
        core::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            phys_to_virt(paddr).as_mut_ptr(),
            file_size,
        );
    }

    Ok(())
}

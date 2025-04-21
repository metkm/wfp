use windows::Win32::{Foundation::HANDLE, NetworkManagement::WindowsFilteringPlatform::{FwpmFilterCreateEnumHandle0, FwpmFilterEnum0, FWPM_FILTER0, FWPM_FILTER_ENUM_TEMPLATE0, FWPM_LAYER_ALE_AUTH_LISTEN_V4}};

pub fn find_matching_filters(engine_handle: &HANDLE) -> Result<Vec<FWPM_FILTER0>, &str> {
    let enum_template = FWPM_FILTER_ENUM_TEMPLATE0 {
        actionMask: 0xFFFFFFFF,
        layerKey: FWPM_LAYER_ALE_AUTH_LISTEN_V4,
        ..Default::default()
    };

    let mut enum_handle = HANDLE::default();
    let create_enum_result = unsafe {
        FwpmFilterCreateEnumHandle0(*engine_handle, Some(&enum_template), &mut enum_handle)
    };

    if create_enum_result != 0 {
        return Err("error creating enum handle");
    }

    let mut filters: *mut *mut FWPM_FILTER0 = std::ptr::null_mut();
    let mut returned_filter_count = 0;

    let filters_enum_result = unsafe {
        FwpmFilterEnum0(
            *engine_handle,
            enum_handle,
            20,
            &mut filters,
            // filters.as_mut_ptr() as *mut _ as *mut _,
            &mut returned_filter_count,
        )
    };

    if filters_enum_result != 0 {
        println!(
            "erorr getting filters enum result {:?}",
            filters_enum_result
        );

        return Err("error enumerating filters");
    }

    let mut filter_objects = Vec::with_capacity(returned_filter_count as usize);

    for i in 0..returned_filter_count {
        let filter = unsafe { **(filters.add(i as usize)) };
        filter_objects.push(filter);
    }
    
    Ok(filter_objects)
}

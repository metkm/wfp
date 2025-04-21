use windows::Win32::{
    Foundation::HANDLE,
    NetworkManagement::WindowsFilteringPlatform::{
        FWPM_NET_EVENT_ENUM_TEMPLATE0, FWPM_NET_EVENT0,
        FwpmNetEventCreateEnumHandle0, FwpmNetEventEnum0,
    },
};

pub fn get_recent_events(engine_handle: &HANDLE, max_event_count: u32) -> Result<Vec<FWPM_NET_EVENT0>, &str> {
    let enum_template = FWPM_NET_EVENT_ENUM_TEMPLATE0 {
        ..Default::default()
    };

    let mut enum_handle = HANDLE::default();

    let enum_handle_result = unsafe {
        FwpmNetEventCreateEnumHandle0(*engine_handle, Some(&enum_template), &mut enum_handle)
    };

    if enum_handle_result != 0 {
        return Err("error creating event enum handle");
    }

    let mut event_p: *mut *mut FWPM_NET_EVENT0 = std::ptr::null_mut();
    let mut events_returned = 0;

    let event_enum_result = unsafe {
        FwpmNetEventEnum0(
            *engine_handle,
            enum_handle,
            max_event_count,
            &mut event_p,
            &mut events_returned,
        )
    };

    if event_enum_result != 0 {
        return Err("error getting enum event results");
    }

    let mut events: Vec<FWPM_NET_EVENT0> = Vec::with_capacity(events_returned as usize);

    for i in 0..events_returned {
        let event = unsafe { **(event_p.add(i as usize)) };
        events.push(event);
    }

    Ok(events)
}

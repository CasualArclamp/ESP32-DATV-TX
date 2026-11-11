//! ESP32-DATV transmitter library: DVB-S/S2 coding, the ESP link and the FFmpeg source.
pub mod av1ts;
pub mod dvbs;
pub mod dvbs2;
pub mod link;
pub mod plan;
pub mod prefs;
pub mod preview;
pub mod source;
pub mod tx;

/// Makes the FFmpeg processes this program starts end with it, however it ends. Windows
/// keeps children running when their parent is killed; an encoder would go on using the
/// processor with nobody reading what it makes. The program puts itself in a job object
/// that kills its members when the last handle to it closes, which is when this process is
/// gone; children are born into the job.
pub fn end_children_with_me() {
    #[cfg(windows)]
    unsafe {
        use windows_sys::Win32::System::JobObjects::*;
        use windows_sys::Win32::System::Threading::GetCurrentProcess;
        let job = CreateJobObjectW(std::ptr::null(), std::ptr::null());
        if job.is_null() {
            return;
        }
        let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
        info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
        SetInformationJobObject(
            job,
            JobObjectExtendedLimitInformation,
            &info as *const _ as *const _,
            std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32,
        );
        AssignProcessToJobObject(job, GetCurrentProcess());
        // the handle is kept until the process ends: closing it is the signal
    }
}

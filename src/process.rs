use serde::Deserialize;

use petgraph::{Direction, Graph};
use std::collections::HashMap;
use std::hash::Hasher;
use wmi::WMIDateTime;

/// The Win32_Process WMI class represents a process on an operating system <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-process>
#[derive(Deserialize, Debug, Clone)]
pub struct Win32_Process {
    pub CreationClassName: Option<String>,
    pub Caption: Option<String>,
    pub CommandLine: Option<String>,
    pub CreationDate: Option<WMIDateTime>,
    pub CSCreationClassName: Option<String>,
    pub CSName: Option<String>,
    pub Description: Option<String>,
    pub ExecutablePath: Option<String>,
    pub ExecutionState: Option<u16>,
    pub Handle: Option<String>,
    pub HandleCount: Option<u32>,
    pub InstallDate: Option<WMIDateTime>,
    pub KernelModeTime: Option<u64>,
    pub MaximumWorkingSetSize: Option<u32>,
    pub MinimumWorkingSetSize: Option<u32>,
    pub Name: Option<String>,
    pub OSCreationClassName: Option<String>,
    pub OSName: Option<String>,
    pub OtherOperationCount: Option<u64>,
    pub OtherTransferCount: Option<u64>,
    pub PageFaults: Option<u32>,
    pub PageFileUsage: Option<u32>,
    pub ParentProcessId: Option<u32>,
    pub PeakPageFileUsage: Option<u32>,
    pub PeakVirtualSize: Option<u64>,
    pub PeakWorkingSetSize: Option<u32>,
    pub Priority: Option<u32>,
    pub PrivatePageCount: Option<u64>,
    pub ProcessId: Option<u32>,
    pub QuotaNonPagedPoolUsage: Option<u32>,
    pub QuotaPagedPoolUsage: Option<u32>,
    pub QuotaPeakNonPagedPoolUsage: Option<u32>,
    pub QuotaPeakPagedPoolUsage: Option<u32>,
    pub ReadOperationCount: Option<u64>,
    pub ReadTransferCount: Option<u64>,
    pub SessionId: Option<u32>,
    // is always null
    pub Status: Option<String>,
    pub TerminationDate: Option<WMIDateTime>,
    pub ThreadCount: Option<u32>,
    pub UserModeTime: Option<u64>,
    pub VirtualSize: Option<u64>,
    pub WindowsVersion: Option<String>,
    pub WorkingSetSize: Option<u64>,
    pub WriteOperationCount: Option<u64>,
    pub WriteTransferCount: Option<u64>,
}

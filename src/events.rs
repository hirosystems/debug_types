use serde::{Deserialize, Serialize};

use crate::types::{Breakpoint, Capabilities, InvalidatedAreas, Module, Source};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Event {
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub body: Option<EventBody>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "event", content = "body")]
pub enum EventBody {
    Initialized,
    Stopped(StoppedEvent),
    Continued(ContinuedEvent),
    Exited(ExitedEvent),
    Terminated(TerminatedEvent),
    Thread(ThreadEvent),
    Output(OutputEvent),
    Breakpoint(BreakpointEvent),
    Module(ModuleEvent),
    LoadedSource(LoadedSourceEvent),
    Process(ProcessEvent),
    Capabilities(CapabilitiesEvent),
    ProgressStart(ProgressStartEvent),
    ProgressUpdate(ProgressUpdateEvent),
    ProgressEnd(ProgressEndEvent),
    Invalidated(InvalidatedEvent),
    Memory(MemoryEvent),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StoppedReason {
    Step,
    Breakpoint,
    Exception,
    Pause,
    Entry,
    Goto,
    #[serde(rename = "function breakpoint")]
    FunctionBreakpoint,
    #[serde(rename = "data breakpoint")]
    DataBreakpoint,
    #[serde(rename = "instruction breakpoint")]
    InstructionBreakpoint,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoppedEvent {
    /**
     * The reason for the event.
     * For backward compatibility this String is shown in the UI if the
     * 'description' attribute is missing (but it must not be translated).
     * Values: 'step', 'breakpoint', 'exception', 'pause', 'entry', 'goto',
     * 'function breakpoint', 'data breakpoint', 'instruction breakpoint', etc.
     */
    reason: StoppedReason,

    /**
     * The full reason for the event, e.g. 'Paused on exception'. This String is
     * shown in the UI as is and must be translated.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /**
     * The thread which was stopped.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i32>,

    /**
     * A value of true hints to the frontend that this event should not change
     * the focus.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_focus_hint: Option<bool>,

    /**
     * Additional information. E.g. if reason is 'exception', text contains the
     * exception name. This String is shown in the UI.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /**
     * If 'allThreadsStopped' is true, a debug adapter can announce that all
     * threads have stopped.
     * - The client should use this information to enable that all threads can
     * be expanded to access their stacktraces.
     * - If the attribute is missing or false, only the thread with the given
     * threadId can be expanded.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_threads_stopped: Option<bool>,

    /**
     * Ids of the breakpoints that triggered the event. In most cases there will
     * be only a single breakpoint but here are some examples for multiple
     * breakpoints:
     * - Different types of breakpoints map to the same location.
     * - Multiple source breakpoints get collapsed to the same instruction by
     * the compiler/runtime.
     * - Multiple function breakpoints with different function names map to the
     * same location.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hit_breakpoint_ids: Option<Vec<i32>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinuedEvent {
    /**
     * The thread which was continued.
     */
    pub thread_id: i32,

    /**
     * If 'allThreadsContinued' is true, a debug adapter can announce that all
     * threads have continued.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_threads_continued: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExitedEvent {
    /**
     * The exit code returned from the debuggee.
     */
    pub exit_code: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminatedEvent {
    /**
     * A debug adapter may set 'restart' to true (or to an arbitrary object) to
     * request that the front end restarts the session.
     * The value is not interpreted by the client and passed unmodified as an
     * attribute '__restart' to the 'launch' and 'attach' requests.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Reason {
    Started,
    Exited,
    Changed,
    New,
    Removed,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadEvent {
    /**
     * The reason for the event.
     * Values: 'started', 'exited', etc.
     */
    pub reason: Reason,

    /**
     * The identifier of the thread.
     */
    pub thread_id: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    Console,
    Important,
    Stdout,
    Stderr,
    Telemetry,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Group {
    Start,
    StartCollapsed,
    End,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputEvent {
    /**
     * The output category. If not specified or if the category is not
     * understand by the client, 'console' is assumed.
     * Values:
     * 'console': Show the output in the client's default message UI, e.g. a
     * 'debug console'. This category should only be used for informational
     * output from the debugger (as opposed to the debuggee).
     * 'important': A hint for the client to show the ouput in the client's UI
     * for important and highly visible information, e.g. as a popup
     * notification. This category should only be used for important messages
     * from the debugger (as opposed to the debuggee). Since this category value
     * is a hint, clients might ignore the hint and assume the 'console'
     * category.
     * 'stdout': Show the output as normal program output from the debuggee.
     * 'stderr': Show the output as error program output from the debuggee.
     * 'telemetry': Send the output to telemetry instead of showing it to the
     * user.
     * etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,

    /**
     * The output to report.
     */
    pub output: String,

    /**
     * Support for keeping an output log organized by grouping related messages.
     * Values:
     * 'start': Start a new group in expanded mode. Subsequent output events are
     * members of the group and should be shown indented.
     * The 'output' attribute becomes the name of the group and is not indented.
     * 'startCollapsed': Start a new group in collapsed mode. Subsequent output
     * events are members of the group and should be shown indented (as soon as
     * the group is expanded).
     * The 'output' attribute becomes the name of the group and is not indented.
     * 'end': End the current group and decreases the indentation of subsequent
     * output events.
     * A non empty 'output' attribute is shown as the unindented end of the
     * group.
     * etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,

    /**
     * If an attribute 'variablesReference' exists and its value is > 0, the
     * output contains objects which can be retrieved by passing
     * 'variablesReference' to the 'variables' request. The value should be less
     * than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<i32>,

    /**
     * An optional source location where the output was produced.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /**
     * An optional source location line where the output was produced.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,

    /**
     * An optional source location column where the output was produced.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /**
     * Optional data to report. For the 'telemetry' category the data will be
     * sent to telemetry, for the other categories the data is shown in JSON
     * format.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointEvent {
    /**
     * The reason for the event.
     * Values: 'changed', 'new', 'removed', etc.
     */
    pub reason: Reason,

    /**
     * The 'id' attribute is used to find the target breakpoint and the other
     * attributes are used as the new values.
     */
    pub breakpoint: Breakpoint,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleEvent {
    /**
     * The reason for the event.
     * Values: 'new', 'changed', 'removed', etc.
     */
    pub reason: Reason,

    /**
     * The new, changed, or removed module. In case of 'removed' only the module
     * id is used.
     */
    pub module: Module,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedSourceEvent {
    /**
     * The reason for the event.
     * Values: 'new', 'changed', 'removed', etc.
     */
    pub reason: Reason,

    /**
     * The new, changed, or removed source.
     */
    pub source: Source,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum StartMethod {
    Launch,
    Attach,
    AttachForSuspendedLaunch,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessEvent {
    /**
     * The logical name of the process. This is usually the full path to
     * process's executable file. Example: /home/example/myproj/program.js.
     */
    pub name: String,

    /**
     * The system process id of the debugged process. This property will be
     * missing for non-system processes.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_process_id: Option<i32>,

    /**
     * If true, the process is running on the same computer as the debug
     * adapter.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_local_process: Option<bool>,

    /**
     * Describes how the debug engine started debugging this process.
     * Values:
     * 'launch': Process was launched under the debugger.
     * 'attach': Debugger attached to an existing process.
     * 'attachForSuspendedLaunch': A project launcher component has launched a
     * new process in a suspended state and then asked the debugger to attach.
     * etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_method: Option<StartMethod>,

    /**
     * The size of a pointer or address for this process, in bits. This value
     * may be used by clients when formatting addresses for display.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pointer_size: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilitiesEvent {
    /**
     * The set of updated capabilities.
     */
    pub capabilities: Capabilities,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressStartEvent {
    /**
     * An ID that must be used in subsequent 'progressUpdate' and 'progressEnd'
     * events to make them refer to the same progress reporting.
     * IDs must be unique within a debug session.
     */
    pub progress_id: String,

    /**
     * Mandatory (short) title of the progress reporting. Shown in the UI to
     * describe the long running operation.
     */
    pub title: String,

    /**
     * The request ID that this progress report is related to. If specified a
     * debug adapter is expected to emit
     * progress events for the long running request until the request has been
     * either completed or cancelled.
     * If the request ID is omitted, the progress report is assumed to be
     * related to some general activity of the debug adapter.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i32>,

    /**
     * If true, the request that reports progress may be canceled with a
     * 'cancel' request.
     * So this property basically controls whether the client should use UX that
     * supports cancellation.
     * Clients that don't support cancellation are allowed to ignore the
     * setting.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,

    /**
     * Optional, more detailed progress message.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /**
     * Optional progress percentage to display (value range: 0 to 100). If
     * omitted no percentage will be shown.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressUpdateEvent {
    /**
     * The ID that was introduced in the initial 'progressStart' event.
     */
    pub progress_id: String,

    /**
     * Optional, more detailed progress message. If omitted, the previous
     * message (if any) is used.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /**
     * Optional progress percentage to display (value range: 0 to 100). If
     * omitted no percentage will be shown.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressEndEvent {
    /**
     * The ID that was introduced in the initial 'ProgressStartEvent'.
     */
    pub progress_id: String,

    /**
     * Optional, more detailed progress message. If omitted, the previous
     * message (if any) is used.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InvalidatedEvent {
    /**
     * Optional set of logical areas that got invalidated. This property has a
     * hint characteristic: a client can only be expected to make a 'best
     * effort' in honouring the areas but there are no guarantees. If this
     * property is missing, empty, or if values are not understand the client
     * should assume a single value 'all'.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<Vec<InvalidatedAreas>>,

    /**
     * If specified, the client only needs to refetch data related to this
     * thread.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_id: Option<i32>,

    /**
     * If specified, the client only needs to refetch data related to this stack
     * frame (and the 'threadId' is ignored).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_frame_id: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemoryEvent {
    /**
     * Memory reference of a memory range that has been updated.
     */
    pub memory_reference: String,

    /**
     * Starting offset in bytes where memory has been updated. Can be negative.
     */
    pub offset: i32,

    /**
     * Number of bytes updated.
     */
    pub count: i32,
}

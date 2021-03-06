use serde::{Deserialize, Serialize};

use crate::types::*;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Response {
    pub request_seq: i64,
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub body: Option<ResponseBody>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "command", content = "body")]
pub enum ResponseBody {
    Initialize(InitializeResponse),
    ConfigurationDone,
    Launch,
    Attach,
    Restart,
    Disconnect,
    Terminate,
    BreakpointLocations(BreakpointLocationsResponse),
    SetBreakpoints(SetBreakpointsResponse),
    SetFunctionBreakpoints(SetFunctionBreakpointsResponse),
    SetExceptionBreakpoints(SetExceptionBreakpointsResponse),
    DataBreakpointInfo(DataBreakpointInfoResponse),
    SetInstructionBreakpoints(SetInstructionBreakpointsResponse),
    Continue(ContinueResponse),
    Next,
    StepIn,
    StepOut,
    StepBack,
    ReverseContinue,
    RestartFrame,
    Goto,
    Pause,
    StackTrace(StackTraceResponse),
    Scopes(ScopesResponse),
    Variables(VariablesResponse),
    SetVariable(SetVariableResponse),
    Source(SourceResponse),
    Threads(ThreadsResponse),
    TerminateThreads,
    Modules(ModulesResponse),
    LoadedSources(LoadedSourcesResponse),
    Evaluate(EvaluateResponse),
    SetExpression(SetExpressionResponse),
    StepInTargets(StepInTargetsResponse),
    GotoTargets(GotoTargetsResponse),
    Completions(CompletionsResponse),
    ExceptionInfo(ExceptionInfoResponse),
    ReadMemory(ReadMemoryResponse),
    WriteMemory(WriteMemoryResponse),
    Disassemble(DisassembleResponse),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResponse {
    /**
     * The capabilities of this debug adapter.
     */
    pub capabilities: Capabilities,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocationsResponse {
    /**
     * Sorted set of possible breakpoint locations.
     */
    pub breakpoints: Vec<BreakpointLocation>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsResponse {
    /**
     * Information about the breakpoints.
     * The array elements are in the same order as the elements of the
     * 'breakpoints' (or the deprecated 'lines') array in the arguments.
     */
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFunctionBreakpointsResponse {
    /**
     * Information about the breakpoints. The array elements correspond to the
     * elements of the 'breakpoints' array.
     */
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExceptionBreakpointsResponse {
    /**
     * Information about the exception breakpoints or filters.
     * The breakpoints returned are in the same order as the elements of the
     * 'filters', 'filterOptions', 'exceptionOptions' arrays in the arguments.
     * If both 'filters' and 'filterOptions' are given, the returned array must
     * start with 'filters' information first, followed by 'filterOptions'
     * information.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<Breakpoint>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpointInfoResponse {
    /**
     * An identifier for the data on which a data breakpoint can be registered
     * with the setDataBreakpoints request or null if no data breakpoint is
     * available.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_id: Option<String>,

    /**
     * UI String that describes on what data the breakpoint is set on or why a
     * data breakpoint is not available.
     */
    pub description: String,

    /**
     * Optional attribute listing the available access types for a potential
     * data breakpoint. A UI frontend could surface this information.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_types: Option<Vec<DataBreakpointAccessType>>,

    /**
     * Optional attribute indicating that a potential data breakpoint could be
     * persisted across sessions.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_persist: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDataBreakpointsResponse {
    /**
     * Information about the data breakpoints. The array elements correspond to
     * the elements of the input argument 'breakpoints' array.
     */
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInstructionBreakpointsResponse {
    /**
     * Information about the breakpoints. The array elements correspond to the
     * elements of the 'breakpoints' array.
     */
    pub breakpoints: Vec<Breakpoint>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueResponse {
    /**
     * The value true (or a missing property) signals to the client that all
     * threads have been resumed. The value false must be returned if not all
     * threads were resumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_threads_continued: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceResponse {
    /**
     * The frames of the stackframe. If the array has length zero, there are no
     * stackframes available.
     * This means that there is no location information available.
     */
    pub stack_frames: Vec<StackFrame>,

    /**
     * The total number of frames available in the stack. If omitted or if
     * totalFrames is larger than the available frames, a client is expected to
     * request frames until a request returns less frames than requested (which
     * indicates the end of the stack). Returning monotonically increasing
     * totalFrames values for subsequent requests can be used to enforce paging
     * in the client.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_frames: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesResponse {
    /**
     * The scopes of the stackframe. If the array has length zero, there are no
     * scopes available.
     */
    pub scopes: Vec<Scope>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablesResponse {
    /**
     * All (or a range) of variables for the given variable reference.
     */
    pub variables: Vec<Variable>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableResponse {
    /**
     * The new value of the variable.
     */
    pub value: String,

    /**
     * The type of the new value. Typically shown in the UI when hovering over
     * the value.
     */
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,

    /**
     * If variablesReference is > 0, the new value is structured and its
     * children can be retrieved by passing variablesReference to the
     * VariablesRequest.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<i32>,

    /**
     * The number of named child variables.
     * The client can use this optional information to present the variables in
     * a paged UI and fetch them in chunks.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i32>,

    /**
     * The number of indexed child variables.
     * The client can use this optional information to present the variables in
     * a paged UI and fetch them in chunks.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceResponse {
    /**
     * Content of the source reference.
     */
    pub content: String,

    /**
     * Optional content type (mime type) of the source.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadsResponse {
    /**
     * All threads.
     */
    pub threads: Vec<Thread>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModulesResponse {
    /**
     * All modules or range of modules.
     */
    pub modules: Vec<Module>,

    /**
     * The total number of modules available.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_modules: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadedSourcesResponse {
    /**
     * Set of loaded sources.
     */
    pub sources: Vec<Source>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateResponse {
    /**
     * The result of the evaluate request.
     */
    pub result: String,

    /**
     * The optional type of the evaluate result.
     * This attribute should only be returned by a debug adapter if the client
     * has passed the value true for the 'supportsVariableType' capability of
     * the 'initialize' request.
     */
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_type: Option<String>,

    /**
     * Properties of a evaluate result that can be used to determine how to
     * render the result in the UI.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<VariablePresentationHint>,

    /**
     * If variablesReference is > 0, the evaluate result is structured and its
     * children can be retrieved by passing variablesReference to the
     * VariablesRequest.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    pub variables_reference: i32,

    /**
     * The number of named child variables.
     * The client can use this optional information to present the variables in
     * a paged UI and fetch them in chunks.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i32>,

    /**
     * The number of indexed child variables.
     * The client can use this optional information to present the variables in
     * a paged UI and fetch them in chunks.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i32>,

    /**
     * Optional memory reference to a location appropriate for this result.
     * For pointer type eval results, this is generally a reference to the
     * memory address contained in the pointer.
     * This attribute should be returned by a debug adapter if the client has
     * passed the value true for the 'supportsMemoryReferences' capability of
     * the 'initialize' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_reference: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExpressionResponse {
    /**
     * The new value of the expression.
     */
    pub value: String,

    /**
     * The optional type of the value.
     * This attribute should only be returned by a debug adapter if the client
     * has passed the value true for the 'supportsVariableType' capability of
     * the 'initialize' request.
     */
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,

    /**
     * Properties of a value that can be used to determine how to render the
     * result in the UI.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<VariablePresentationHint>,

    /**
     * If variablesReference is > 0, the value is structured and its children
     * can be retrieved by passing variablesReference to the VariablesRequest.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<i32>,

    /**
     * The number of named child variables.
     * The client can use this optional information to present the variables in
     * a paged UI and fetch them in chunks.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_variables: Option<i32>,

    /**
     * The number of indexed child variables.
     * The client can use this optional information to present the variables in
     * a paged UI and fetch them in chunks.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexed_variables: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInTargetsResponse {
    /**
     * The possible stepIn targets of the specified source location.
     */
    pub targets: Vec<StepInTarget>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoTargetsResponse {
    /**
     * The possible goto targets of the specified location.
     */
    pub targets: Vec<GotoTarget>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionsResponse {
    /**
     * The possible completions for .
     */
    pub targets: Vec<CompletionItem>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionInfoResponse {
    /**
     * ID of the exception that was thrown.
     */
    pub exception_id: String,

    /**
     * Descriptive text for the exception provided by the debug adapter.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /**
     * Mode that caused the exception notification to be raised.
     */
    pub break_mode: ExceptionBreakMode,

    /**
     * Detailed information about the exception.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ExceptionDetails>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadMemoryResponse {
    /**
     * The address of the first byte of data returned.
     * Treated as a hex value if prefixed with '0x', or as a decimal value
     * otherwise.
     */
    pub address: String,

    /**
     * The number of unreadable bytes encountered after the last successfully
     * read byte.
     * This can be used to determine the number of bytes that must be skipped
     * before a subsequent 'readMemory' request will succeed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreadable_bytes: Option<i32>,

    /**
     * The bytes read from memory, encoded using base64.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteMemoryResponse {
    /**
     * Optional property that should be returned when 'allowPartial' is true to
     * indicate the offset of the first byte of data successfully written. Can
     * be negative.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /**
     * Optional property that should be returned when 'allowPartial' is true to
     * indicate the number of bytes starting from address that were successfully
     * written.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_written: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleResponse {
    /**
     * The list of disassembled instructions.
     */
    pub instructions: Vec<DisassembledInstruction>,
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase", tag = "command", content = "arguments")]
pub enum RequestCommand {
    Initialize(InitializeRequestArguments),
    ConfigurationDone,
    Launch(LaunchRequestArguments),
    Attach(AttachRequestArguments),
    Restart(LaunchRequestArguments),
    Disconnect(DisconnectArguments),
    Terminate(TerminateArguments),
    BreakpointLocations(BreakpointLocationsArguments),
    SetBreakpoints(SetBreakpointsArguments),
    SetFunctionBreakpoints(SetFunctionBreakpointsArguments),
    SetExceptionBreakpoints(SetExceptionBreakpointsArguments),
    DataBreakpointInfo(DataBreakpointInfoArguments),
    SetDataBreakpoints(SetDataBreakpointsArguments),
    SetInstructionBreakpoints(SetInstructionBreakpointsArguments),
    Continue(ContinueArguments),
    Next(NextArguments),
    StepIn(StepInArguments),
    StepOut(StepOutArguments),
    StepBack(StepBackArguments),
    ReverseContinue(ReverseContinueArguments),
    RestartFrame(RestartFrameArguments),
    Goto(GotoArguments),
    Pause(PauseArguments),
    StackTrace(StackTraceArguments),
    Scopes(ScopesArguments),
    Variables(VariablesArguments),
    SetVariable(SetVariableArguments),
    Source(SourceArguments),
    Threads,
    TerminateThreads(TerminateThreadsArguments),
    Modules(ModulesArguments),
    LoadedSources,
    Evaluate(EvaluateArguments),
    SetExpression(SetExpressionArguments),
    StepInTargets(StepInTargetsArguments),
    GotoTargets(GotoTargetsArguments),
    Completions(CompletionsArguments),
    ExceptionInfo(ExceptionInfoArguments),
    ReadMemory(ReadMemoryArguments),
    WriteMemory(WriteMemoryArguments),
    Disassemble(DisassembleArguments),
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum TerminalKind {
    Integrated,
    External,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunInTerminalRequestArguments {
    /**
     * What kind of terminal to launch.
     * Values: 'integrated', 'external', etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<TerminalKind>,

    /**
     * Optional title of the terminal.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /**
     * Working directory for the command. For non-empty, valid paths this
     * typically results in execution of a change directory command.
     */
    pub cwd: String,

    /**
     * List of arguments. The first argument is the command to run.
     */
    pub args: Vec<String>,

    /**
     * Environment key-value pairs that are added to or removed from the default
     * environment.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, Option<String>>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum PathFormat {
    Path,
    Uri,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeRequestArguments {
    /**
     * The ID of the (frontend) client using this adapter.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,

    /**
     * The human readable name of the (frontend) client using this adapter.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,

    /**
     * The ID of the debug adapter.
     */
    pub adapter_id: String,

    /**
     * The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
     * or de-CH.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,

    /**
     * If true all line numbers are 1-based (default).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines_start_at1: Option<bool>,

    /**
     * If true all column numbers are 1-based (default).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns_start_at1: Option<bool>,

    /**
     * Determines in what format paths are specified. The default is 'path', which
     * is the native format.
     * Values: 'path', 'uri', etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_format: Option<PathFormat>,

    /**
     * Client supports the optional type attribute for variables.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_variable_type: Option<bool>,

    /**
     * Client supports the paging of variables.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_variable_paging: Option<bool>,

    /**
     * Client supports the runInTerminal request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_run_in_terminal_request: Option<bool>,

    /**
     * Client supports memory references.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_memory_references: Option<bool>,

    /**
     * Client supports progress reporting.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_progress_reporting: Option<bool>,

    /**
     * Client supports the invalidated event.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_invalidated_event: Option<bool>,

    /**
     * Client supports the memory event.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_memory_event: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchRequestArguments {
    /**
     * If noDebug is true the launch request should launch the program without
     * enabling debugging.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_debug: Option<bool>,

    /**
     * Optional data from the previous, restarted session.
     * The data is sent as the 'restart' attribute of the 'terminated' event.
     * The client should leave the data intact.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    __restart: Option<String>,

    /**
     * Type of debug session launched (from launch.json).
     */
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,

    /**
     * Name of this debug session.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /**
     * Path to the manifest file (Clarinet.toml) for the project to be debugged.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,

    /**
     * Expression to be debugged.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,

    /**
     * Some kind of configuration identifier.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub __configuration_target: Option<i64>,

    /**
     * Unique identifier for this debug session.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub __session_id: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AttachRequestArguments {
    /**
     * Optional data from the previous, restarted session.
     * The data is sent as the 'restart' attribute of the 'terminated' event.
     * The client should leave the data intact.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    __restart: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestartArguments {
    /**
     * The latest version of the 'launch' or 'attach' configuration.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<LaunchRequestArguments>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisconnectArguments {
    /**
     * A value of true indicates that this 'disconnect' request is part of a
     * restart sequence.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,

    /**
     * Indicates whether the debuggee should be terminated when the debugger is
     * disconnected.
     * If unspecified, the debug adapter is free to do whatever it thinks is best.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportTerminateDebuggee' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_debuggee: Option<bool>,

    /**
     * Indicates whether the debuggee should stay suspended when the debugger is
     * disconnected.
     * If unspecified, the debuggee should resume execution.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportSuspendDebuggee' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend_debuggee: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateArguments {
    /**
     * A value of true indicates that this 'terminate' request is part of a
     * restart sequence.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocationsArguments {
    /**
     * The source location of the breakpoints; either 'source.path' or
     * 'source.reference' must be specified.
     */
    pub source: Source,

    /**
     * Start line of range to search possible breakpoint locations in. If only the
     * line is specified, the request returns all possible locations in that line.
     */
    pub line: i32,

    /**
     * Optional start column of range to search possible breakpoint locations in.
     * If no start column is given, the first column in the start line is assumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /**
     * Optional end line of range to search possible breakpoint locations in. If
     * no end line is given, then the end line is assumed to be the start line.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_line: Option<i32>,

    /**
     * Optional end column of range to search possible breakpoint locations in. If
     * no end column is given, then it is assumed to be in the last column of the
     * end line.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_column: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsArguments {
    /**
     * The source location of the breakpoints; either 'source.path' or
     * 'source.reference' must be specified.
     */
    pub source: Source,

    /**
     * The code locations of the breakpoints.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakpoints: Option<Vec<SourceBreakpoint>>,

    /**
     * Deprecated: The code locations of the breakpoints.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<i32>>,

    /**
     * A value of true indicates that the underlying source has been modified
     * which results in new breakpoint locations.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_modified: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetFunctionBreakpointsArguments {
    /**
     * The function names of the breakpoints.
     */
    pub breakpoints: Vec<FunctionBreakpoint>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExceptionBreakpointsArguments {
    /**
     * Set of exception filters specified by their ID. The set of all possible
     * exception filters is defined by the 'exceptionBreakpointFilters'
     * capability. The 'filter' and 'filterOptions' sets are additive.
     */
    pub filters: Vec<String>,

    /**
     * Set of exception filters and their options. The set of all possible
     * exception filters is defined by the 'exceptionBreakpointFilters'
     * capability. This attribute is only honored by a debug adapter if the
     * capability 'supportsExceptionFilterOptions' is true. The 'filter' and
     * 'filterOptions' sets are additive.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_options: Option<Vec<ExceptionFilterOptions>>,

    /**
     * Configuration options for selected exceptions.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsExceptionOptions' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception_options: Option<Vec<ExceptionOptions>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpointInfoArguments {
    /**
     * Reference to the Variable container if the data breakpoint is requested for
     * a child of the container.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables_reference: Option<i32>,

    /**
     * The name of the Variable's child to obtain data breakpoint information for.
     * If variablesReference isn't provided, this can be an expression.
     */
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDataBreakpointsArguments {
    /**
     * The contents of this array replaces all existing data breakpoints. An empty
     * array clears all data breakpoints.
     */
    pub breakpoints: Vec<DataBreakpoint>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetInstructionBreakpointsArguments {
    /**
     * The instruction references of the breakpoints
     */
    pub breakpoints: Vec<InstructionBreakpoint>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContinueArguments {
    /**
     * Specifies the active thread. If the debug adapter supports single thread
     * execution (see 'supportsSingleThreadExecutionRequests') and the optional
     * argument 'singleThread' is true, only the thread with this ID is resumed.
     */
    pub thread_id: i32,

    /**
     * If this optional flag is true, execution is resumed only for the thread
     * with given 'threadId'.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_thread: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NextArguments {
    /**
     * Specifies the thread for which to resume execution for one step (of the
     * given granularity).
     */
    pub thread_id: i32,

    /**
     * If this optional flag is true, all other suspended threads are not resumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_thread: Option<bool>,

    /**
     * Optional granularity to step. If no granularity is specified, a granularity
     * of 'statement' is assumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<SteppingGranularity>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInArguments {
    /**
     * Specifies the thread for which to resume execution for one step-into (of
     * the given granularity).
     */
    pub thread_id: i32,

    /**
     * If this optional flag is true, all other suspended threads are not resumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_thread: Option<bool>,

    /**
     * Optional id of the target to step into.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<i32>,

    /**
     * Optional granularity to step. If no granularity is specified, a granularity
     * of 'statement' is assumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<SteppingGranularity>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepOutArguments {
    /**
     * Specifies the thread for which to resume execution for one step-out (of the
     * given granularity).
     */
    pub thread_id: i32,

    /**
     * If this optional flag is true, all other suspended threads are not resumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_thread: Option<bool>,

    /**
     * Optional granularity to step. If no granularity is specified, a granularity
     * of 'statement' is assumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<SteppingGranularity>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepBackArguments {
    /**
     * Specifies the thread for which to resume execution for one step backwards
     * (of the given granularity).
     */
    pub thread_id: i32,

    /**
     * If this optional flag is true, all other suspended threads are not resumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_thread: Option<bool>,

    /**
     * Optional granularity to step. If no granularity is specified, a granularity
     * of 'statement' is assumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<SteppingGranularity>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseContinueArguments {
    /**
     * Specifies the active thread. If the debug adapter supports single thread
     * execution (see 'supportsSingleThreadExecutionRequests') and the optional
     * argument 'singleThread' is true, only the thread with this ID is resumed.
     */
    pub thread_id: i32,

    /**
     * If this optional flag is true, backward execution is resumed only for the
     * thread with given 'threadId'.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_thread: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RestartFrameArguments {
    /**
     * Restart this stackframe.
     */
    pub frame_id: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoArguments {
    /**
     * Set the goto target for this thread.
     */
    pub thread_id: i32,

    /**
     * The location where the debuggee will continue to run.
     */
    pub target_id: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PauseArguments {
    /**
     * Pause execution for this thread.
     */
    pub thread_id: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceArguments {
    /**
     * Retrieve the stacktrace for this thread.
     */
    pub thread_id: i32,

    /**
     * The index of the first frame to return; if omitted frames start at 0.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_frame: Option<i32>,

    /**
     * The maximum number of frames to return. If levels is not specified or 0,
     * all frames are returned.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub levels: Option<i32>,

    /**
     * Specifies details on how to format the stack frames.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsValueFormattingOptions' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<StackFrameFormat>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScopesArguments {
    /**
     * Retrieve the scopes for this stackframe.
     */
    pub frame_id: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariableFilter {
    Indexed,
    Named,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablesArguments {
    /**
     * The Variable reference.
     */
    pub variables_reference: i32,

    /**
     * Optional filter to limit the child variables to either named or indexed. If
     * omitted, both types are fetched.
     * Values: 'indexed', 'named', etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<VariableFilter>,

    /**
     * The index of the first variable to return; if omitted children start at 0.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,

    /**
     * The number of variables to return. If count is missing or 0, all variables
     * are returned.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,

    /**
     * Specifies details on how to format the Variable values.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsValueFormattingOptions' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableArguments {
    /**
     * The reference of the variable container.
     */
    pub variables_reference: i32,

    /**
     * The name of the variable in the container.
     */
    pub name: String,

    /**
     * The value of the variable.
     */
    pub value: String,

    /**
     * Specifies details on how to format the response value.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceArguments {
    /**
     * Specifies the source content to load. Either source.path or
     * source.sourceReference must be specified.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /**
     * The reference to the source. This is the same as source.sourceReference.
     * This is provided for backward compatibility since old backends do not
     * understand the 'source' attribute.
     */
    pub source_reference: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TerminateThreadsArguments {
    /**
     * Ids of threads to be terminated.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thread_ids: Option<Vec<i32>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModulesArguments {
    /**
     * The index of the first module to return; if omitted modules start at 0.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_module: Option<i32>,

    /**
     * The number of modules to return. If moduleCount is not specified or 0, all
     * modules are returned.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_count: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum EvalContext {
    Variables,
    Watch,
    Repl,
    Hover,
    Clipboard,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateArguments {
    /**
     * The expression to evaluate.
     */
    pub expression: String,

    /**
     * Evaluate the expression in the scope of this stack frame. If not specified,
     * the expression is evaluated in the global scope.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<i32>,

    /**
     * The context in which the evaluate request is used.
     * Values:
     * 'variables': evaluate is called from a variables view context.
     * 'watch': evaluate is called from a watch view context.
     * 'repl': evaluate is called from a REPL context.
     * 'hover': evaluate is called to generate the debug hover contents.
     * This value should only be used if the capability
     * 'supportsEvaluateForHovers' is true.
     * 'clipboard': evaluate is called to generate clipboard contents.
     * This value should only be used if the capability 'supportsClipboardContext'
     * is true.
     * etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<EvalContext>,

    /**
     * Specifies details on how to format the result.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsValueFormattingOptions' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetExpressionArguments {
    /**
     * The l-value expression to assign to.
     */
    pub expression: String,

    /**
     * The value expression to assign to the l-value expression.
     */
    pub value: String,

    /**
     * Evaluate the expressions in the scope of this stack frame. If not
     * specified, the expressions are evaluated in the global scope.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<i32>,

    /**
     * Specifies how the resulting value should be formatted.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ValueFormat>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInTargetsArguments {
    /**
     * The stack frame for which to retrieve the possible stepIn targets.
     */
    pub frame_id: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoTargetsArguments {
    /**
     * The source location for which the goto targets are determined.
     */
    pub source: Source,

    /**
     * The line location for which the goto targets are determined.
     */
    pub line: i32,

    /**
     * An optional column location for which the goto targets are determined.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionsArguments {
    /**
     * Returns completions in the scope of this stack frame. If not specified, the
     * completions are returned for the global scope.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_id: Option<i32>,

    /**
     * One or more source lines. Typically this is the text a user has typed into
     * the debug console before he asked for completion.
     */
    pub text: String,

    /**
     * The character position for which to determine the completion proposals.
     */
    pub column: i32,

    /**
     * An optional line for which to determine the completion proposals. If
     * missing the first line of the text is assumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionInfoArguments {
    /**
     * Thread for which exception information should be retrieved.
     */
    pub thread_id: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadMemoryArguments {
    /**
     * Memory reference to the base location from which data should be read.
     */
    pub memory_reference: String,

    /**
     * Optional offset (in bytes) to be applied to the reference location before
     * reading data. Can be negative.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /**
     * Number of bytes to read at the specified location and offset.
     */
    pub count: i32,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WriteMemoryArguments {
    /**
     * Memory reference to the base location to which data should be written.
     */
    pub memory_reference: String,

    /**
     * Optional offset (in bytes) to be applied to the reference location before
     * writing data. Can be negative.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /**
     * Optional property to control partial writes. If true, the debug adapter
     * should attempt to write memory even if the entire memory region is not
     * writable. In such a case the debug adapter should stop after hitting the
     * first byte of memory that cannot be written and return the number of bytes
     * written in the response via the 'offset' and 'bytesWritten' properties.
     * If false or missing, a debug adapter should attempt to verify the region is
     * writable before writing, and fail the response if it is not.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_partial: Option<bool>,

    /**
     * Bytes to write, encoded using base64.
     */
    pub data: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleArguments {
    /**
     * Memory reference to the base location containing the instructions to
     * disassemble.
     */
    pub memory_reference: String,

    /**
     * Optional offset (in bytes) to be applied to the reference location before
     * disassembling. Can be negative.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /**
     * Optional offset (in instructions) to be applied after the byte offset (if
     * any) before disassembling. Can be negative.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_offset: Option<i32>,

    /**
     * Number of instructions to disassemble starting at the specified location
     * and offset.
     * An adapter must return exactly this number of instructions - any
     * unavailable instructions should be replaced with an implementation-defined
     * 'invalid instruction' value.
     */
    pub instruction_count: i32,

    /**
     * If true, the adapter should attempt to resolve memory addresses and other
     * values to symbolic names.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolve_symbols: Option<bool>,
}

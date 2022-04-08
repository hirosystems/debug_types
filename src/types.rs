use crate::NumberOrString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /**
     * The debug adapter supports the 'configurationDone' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsConfigurationDoneRequest: Option<bool>,

    /**
     * The debug adapter supports function breakpoints.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsFunctionBreakpoints: Option<bool>,

    /**
     * The debug adapter supports conditional breakpoints.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsConditionalBreakpoints: Option<bool>,

    /**
     * The debug adapter supports breakpoints that break execution after a
     * specified number of hits.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsHitConditionalBreakpoints: Option<bool>,

    /**
     * The debug adapter supports a (side effect free) evaluate request for data
     * hovers.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsEvaluateForHovers: Option<bool>,

    /**
     * Available exception filter options for the 'setExceptionBreakpoints'
     * request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionBreakpointFilters: Option<Vec<ExceptionBreakpointsFilter>>,

    /**
     * The debug adapter supports stepping back via the 'stepBack' and
     * 'reverseContinue' requests.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsStepBack: Option<bool>,

    /**
     * The debug adapter supports setting a variable to a value.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsSetVariable: Option<bool>,

    /**
     * The debug adapter supports restarting a frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsRestartFrame: Option<bool>,

    /**
     * The debug adapter supports the 'gotoTargets' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsGotoTargetsRequest: Option<bool>,

    /**
     * The debug adapter supports the 'stepInTargets' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsStepInTargetsRequest: Option<bool>,

    /**
     * The debug adapter supports the 'completions' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsCompletionsRequest: Option<bool>,

    /**
     * The set of characters that should trigger completion in a REPL. If not
     * specified, the UI should assume the '.' character.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completionTriggerCharacters: Option<Vec<String>>,

    /**
     * The debug adapter supports the 'modules' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsModulesRequest: Option<bool>,

    /**
     * The set of additional module information exposed by the debug adapter.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additionalModuleColumns: Option<Vec<ColumnDescriptor>>,

    /**
     * Checksum algorithms supported by the debug adapter.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportedChecksumAlgorithms: Option<Vec<ChecksumAlgorithm>>,

    /**
     * The debug adapter supports the 'restart' request. In this case a client
     * should not implement 'restart' by terminating and relaunching the adapter
     * but by calling the RestartRequest.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsRestartRequest: Option<bool>,

    /**
     * The debug adapter supports 'exceptionOptions' on the
     * setExceptionBreakpoints request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsExceptionOptions: Option<bool>,

    /**
     * The debug adapter supports a 'format' attribute on the stackTraceRequest,
     * variablesRequest, and evaluateRequest.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsValueFormattingOptions: Option<bool>,

    /**
     * The debug adapter supports the 'exceptionInfo' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsExceptionInfoRequest: Option<bool>,

    /**
     * The debug adapter supports the 'terminateDebuggee' attribute on the
     * 'disconnect' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportTerminateDebuggee: Option<bool>,

    /**
     * The debug adapter supports the 'suspendDebuggee' attribute on the
     * 'disconnect' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportSuspendDebuggee: Option<bool>,

    /**
     * The debug adapter supports the delayed loading of parts of the stack, which
     * requires that both the 'startFrame' and 'levels' arguments and an optional
     * 'totalFrames' result of the 'StackTrace' request are supported.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsDelayedStackTraceLoading: Option<bool>,

    /**
     * The debug adapter supports the 'loadedSources' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsLoadedSourcesRequest: Option<bool>,

    /**
     * The debug adapter supports logpoints by interpreting the 'logMessage'
     * attribute of the SourceBreakpoint.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsLogPoints: Option<bool>,

    /**
     * The debug adapter supports the 'terminateThreads' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsTerminateThreadsRequest: Option<bool>,

    /**
     * The debug adapter supports the 'setExpression' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsSetExpression: Option<bool>,

    /**
     * The debug adapter supports the 'terminate' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsTerminateRequest: Option<bool>,

    /**
     * The debug adapter supports data breakpoints.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsDataBreakpoints: Option<bool>,

    /**
     * The debug adapter supports the 'readMemory' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsReadMemoryRequest: Option<bool>,

    /**
     * The debug adapter supports the 'writeMemory' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsWriteMemoryRequest: Option<bool>,

    /**
     * The debug adapter supports the 'disassemble' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsDisassembleRequest: Option<bool>,

    /**
     * The debug adapter supports the 'cancel' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsCancelRequest: Option<bool>,

    /**
     * The debug adapter supports the 'breakpointLocations' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsBreakpointLocationsRequest: Option<bool>,

    /**
     * The debug adapter supports the 'clipboard' context value in the 'evaluate'
     * request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsClipboardContext: Option<bool>,

    /**
     * The debug adapter supports stepping granularities (argument 'granularity')
     * for the stepping requests.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsSteppingGranularity: Option<bool>,

    /**
     * The debug adapter supports adding breakpoints based on instruction
     * references.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsInstructionBreakpoints: Option<bool>,

    /**
     * The debug adapter supports 'filterOptions' as an argument on the
     * 'setExceptionBreakpoints' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsExceptionFilterOptions: Option<bool>,

    /**
     * The debug adapter supports the 'singleThread' property on the execution
     * requests ('continue', 'next', 'stepIn', 'stepOut', 'reverseContinue',
     * 'stepBack').
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsSingleThreadExecutionRequests: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionBreakpointsFilter {
    /**
     * The internal ID of the filter option. This value is passed to the
     * 'setExceptionBreakpoints' request.
     */
    pub filter: String,

    /**
     * The name of the filter option. This will be shown in the UI.
     */
    pub label: String,

    /**
     * An optional help text providing additional information about the exception
     * filter. This String is typically shown as a hover and must be translated.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /**
     * Initial value of the filter option. If not specified a value 'false' is
     * assumed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<bool>,

    /**
     * Controls whether a condition can be specified for this filter option. If
     * false or missing, a condition can not be set.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supportsCondition: Option<bool>,

    /**
     * An optional help text providing information about the condition. This
     * String is shown as the placeholder text for a text box and must be
     * translated.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditionDescription: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    /**
     * Unique identifier for the message.
     */
    pub id: i32,

    /**
     * A format String for the message. Embedded variables have the form '{name}'.
     * If variable name starts with an underscore character, the variable does not
     * contain user data (PII) and can be safely used for telemetry purposes.
     */
    pub format: String,

    /**
     * An object used as a dictionary for looking up the variables in the format
     * String.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, String>>,

    /**
     * If true send to telemetry.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sendTelemetry: Option<bool>,

    /**
     * If true show user.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub showUser: Option<bool>,

    /**
     * An optional url where additional information about this message can be
     * found.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /**
     * An optional label that is presented to the user as the UI for opening the
     * url.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlLabel: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Module {
    /**
     * Unique identifier for the module.
     */
    pub id: NumberOrString,

    /**
     * A name of the module.
     */
    pub name: String,

    /**
     * optional but recommended attributes.
     * always try to use these first before introducing additional attributes.
     *
     * Logical full path to the module. The exact definition is implementation
     * defined, but usually this would be a full path to the on-disk file for the
     * module.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /**
     * True if the module is optimized.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isOptimized: Option<bool>,

    /**
     * True if the module is considered 'user code' by a debugger that supports
     * 'Just My Code'.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isUserCode: Option<bool>,

    /**
     * Version of Module.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /**
     * User understandable description of if symbols were found for the module
     * (ex: 'Symbols Loaded', 'Symbols not found', etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbolStatus: Option<String>,

    /**
     * Logical full path to the symbol file. The exact definition is
     * implementation defined.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbolFilePath: Option<String>,

    /**
     * Module created or modified.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateTimeStamp: Option<String>,

    /**
     * Address range covered by this module.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addressRange: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColumnType {
    String,
    Number,
    Boolean,
    #[serde(rename = "unixTimestampUTC")]
    UnixTimestampUTC,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDescriptor {
    /**
     * Name of the attribute rendered in this column.
     */
    pub attributeName: String,

    /**
     * Header UI label of column.
     */
    pub label: String,

    /**
     * Format to use for the rendered values in this column. TBD how the format
     * Strings looks like.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /**
     * Datatype of values in this column.  Defaults to 'String' if not specified.
     * Values: 'String', 'number', 'boolean', 'unixTimestampUTC', etc.
     */
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_type: Option<ColumnType>,

    /**
     * Width of this column in characters (hint only).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModulesViewDescriptor {
    pub columns: Vec<ColumnDescriptor>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Thread {
    /**
     * Unique identifier for the thread.
     */
    pub id: i32,

    /**
     * A name of the thread.
     */
    pub name: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PresentationHint {
    Normal,
    Emphasize,
    Deemphasize,
    Label,
    Subtle,
    Arguments,
    Locals,
    Registers,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /**
     * The short name of the source. Every source returned from the debug adapter
     * has a name.
     * When sending a source to the debug adapter this name is optional.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /**
     * The path of the source to be shown in the UI.
     * It is only used to locate and load the content of the source if no
     * sourceReference is specified (or its value is 0).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /**
     * If sourceReference > 0 the contents of the source must be retrieved through
     * the SourceRequest (even if a path is specified).
     * A sourceReference is only valid for a session, so it must not be used to
     * persist a source.
     * The value should be less than or equal to 2147483647 (2^31-1).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourceReference: Option<i32>,

    /**
     * An optional hint for how to present the source in the UI.
     * A value of 'deemphasize' can be used to indicate that the source is not
     * available or that it is skipped on stepping.
     * Values: 'normal', 'emphasize', 'deemphasize', etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentationHint: Option<PresentationHint>,

    /**
     * The (optional) origin of this source: possible values 'internal module',
     * 'inlined content from source map', etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,

    /**
     * An optional list of sources that are related to this source. These may be
     * the source that generated this source.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<Source>>,

    /**
     * Optional data that a debug adapter might want to loop through the client.
     * The client should leave the data intact and persist it across sessions. The
     * client should not interpret the data.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapterData: Option<String>,

    /**
     * The checksums associated with this file.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksums: Option<Vec<Checksum>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackFrame {
    /**
     * An identifier for the stack frame. It must be unique across all threads.
     * This id can be used to retrieve the scopes of the frame with the
     * 'scopesRequest' or to restart the execution of a stackframe.
     */
    pub id: i32,

    /**
     * The name of the stack frame, typically a method name.
     */
    pub name: String,

    /**
     * The optional source of the frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /**
     * The line within the file of the frame. If source is null or doesn't exist,
     * line is 0 and must be ignored.
     */
    pub line: i32,

    /**
     * The column within the line. If source is null or doesn't exist, column is 0
     * and must be ignored.
     */
    pub column: i32,

    /**
     * An optional end line of the range covered by the stack frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endLine: Option<i32>,

    /**
     * An optional end column of the range covered by the stack frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endColumn: Option<i32>,

    /**
     * Indicates whether this frame can be restarted with the 'restart' request.
     * Clients should only use this if the debug adapter supports the 'restart'
     * request (capability 'supportsRestartRequest' is true).
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canRestart: Option<bool>,

    /**
     * Optional memory reference for the current instruction pointer in this
     * frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructionPointerReference: Option<String>,

    /**
     * The module associated with this frame, if any.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moduleId: Option<NumberOrString>,

    /**
     * An optional hint for how to present this frame in the UI.
     * A value of 'label' can be used to indicate that the frame is an artificial
     * frame that is used as a visual label or separator. A value of 'subtle' can
     * be used to change the appearance of a frame in a 'subtle' way.
     * Values: 'normal', 'label', 'subtle', etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentationHint: Option<PresentationHint>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    /**
     * Name of the scope such as 'Arguments', 'Locals', or 'Registers'. This
     * String is shown in the UI as is and can be translated.
     */
    pub name: String,

    /**
     * An optional hint for how to present this scope in the UI. If this attribute
     * is missing, the scope is shown with a generic UI.
     * Values:
     * 'arguments': Scope contains method arguments.
     * 'locals': Scope contains local variables.
     * 'registers': Scope contains registers. Only a single 'registers' scope
     * should be returned from a 'scopes' request.
     * etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentationHint: Option<PresentationHint>,

    /**
     * The variables of this scope can be retrieved by passing the value of
     * variablesReference to the VariablesRequest.
     */
    pub variablesReference: i32,

    /**
     * The number of named variables in this scope.
     * The client can use this optional information to present the variables in a
     * paged UI and fetch them in chunks.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namedVariables: Option<i32>,

    /**
     * The number of indexed variables in this scope.
     * The client can use this optional information to present the variables in a
     * paged UI and fetch them in chunks.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexedVariables: Option<i32>,

    /**
     * If true, the number of variables in this scope is large or expensive to
     * retrieve.
     */
    pub expensive: bool,

    /**
     * Optional source for this scope.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /**
     * Optional start line of the range covered by this scope.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,

    /**
     * Optional start column of the range covered by this scope.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /**
     * Optional end line of the range covered by this scope.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endLine: Option<i32>,

    /**
     * Optional end column of the range covered by this scope.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endColumn: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Variable {
    /**
     * The variable's name.
     */
    pub name: String,

    /**
     * The variable's value.
     * This can be a multi-line text, e.g. for a function the body of a function.
     * For structured variables (which do not have a simple value), it is
     * recommended to provide a one line representation of the structured object.
     * This helps to identify the structured object in the collapsed state when
     * its children are not yet visible.
     * An empty String can be used if no value should be shown in the UI.
     */
    pub value: String,

    /**
     * The type of the variable's value. Typically shown in the UI when hovering
     * over the value.
     * This attribute should only be returned by a debug adapter if the client has
     * passed the value true for the 'supportsVariableType' capability of the
     * 'initialize' request.
     */
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub var_type: Option<String>,

    /**
     * Properties of a variable that can be used to determine how to render the
     * variable in the UI.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentationHint: Option<VariablePresentationHint>,

    /**
     * Optional evaluatable name of this variable which can be passed to the
     * 'EvaluateRequest' to fetch the variable's value.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluateName: Option<String>,

    /**
     * If variablesReference is > 0, the variable is structured and its children
     * can be retrieved by passing variablesReference to the VariablesRequest.
     */
    pub variablesReference: i32,

    /**
     * The number of named child variables.
     * The client can use this optional information to present the children in a
     * paged UI and fetch them in chunks.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namedVariables: Option<i32>,

    /**
     * The number of indexed child variables.
     * The client can use this optional information to present the children in a
     * paged UI and fetch them in chunks.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexedVariables: Option<i32>,

    /**
     * Optional memory reference for the variable if the variable represents
     * executable code, such as a function pointer.
     * This attribute is only required if the client has passed the value true for
     * the 'supportsMemoryReferences' capability of the 'initialize' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memoryReference: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariableKind {
    Property,
    Method,
    Class,
    Data,
    Event,
    BaseClass,
    InnerClass,
    Interface,
    MostDerivedClass,
    Virtual,
    DataBreakpoint,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum VariableAttribute {
    Static,
    Constant,
    ReadOnly,
    RawString,
    HasObjectId,
    CanHaveObjectId,
    HasSideEffects,
    HasDataBreakpoint,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
    Final,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VariablePresentationHint {
    /**
     * The kind of variable. Before introducing additional values, try to use the
     * listed values.
     * Values:
     * 'property': Indicates that the object is a property.
     * 'method': Indicates that the object is a method.
     * 'class': Indicates that the object is a class.
     * 'data': Indicates that the object is data.
     * 'event': Indicates that the object is an event.
     * 'baseClass': Indicates that the object is a base class.
     * 'innerClass': Indicates that the object is an inner class.
     * 'pub struct': Indicates that the object is an interface.
     * 'mostDerivedClass': Indicates that the object is the most derived class.
     * 'virtual': Indicates that the object is virtual, that means it is a
     * synthetic object introducedby the
     * adapter for rendering purposes, e.g. an index range for large arrays.
     * 'dataBreakpoint': Deprecated: Indicates that a data breakpoint is
     * registered for the object. The 'hasDataBreakpoint' attribute should
     * generally be used instead.
     * etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<VariableKind>,

    /**
     * Set of attributes represented as an array of Strings. Before introducing
     * additional values, try to use the listed values.
     * Values:
     * 'static': Indicates that the object is static.
     * 'constant': Indicates that the object is a constant.
     * 'readOnly': Indicates that the object is read only.
     * 'rawString': Indicates that the object is a raw String.
     * 'hasObjectId': Indicates that the object can have an Object ID created for
     * it.
     * 'canHaveObjectId': Indicates that the object has an Object ID associated
     * with it.
     * 'hasSideEffects': Indicates that the evaluation had side effects.
     * 'hasDataBreakpoint': Indicates that the object has its value tracked by a
     * data breakpoint.
     * etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<Vec<VariableAttribute>>,

    /**
     * Visibility of variable. Before introducing additional values, try to use
     * the listed values.
     * Values: 'public', 'private', 'protected', 'internal', 'final', etc.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<Visibility>,

    /**
     * If true, clients can present the variable with a UI that supports a
     * specific gesture to trigger its evaluation.
     * This mechanism can be used for properties that require executing code when
     * retrieving their value and where the code execution can be expensive and/or
     * produce side-effects. A typical example are properties based on a getter
     * function.
     * Please note that in addition to the 'lazy' flag, the variable's
     * 'variablesReference' must refer to a variable that will provide the value
     * through another 'variable' request.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lazy: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BreakpointLocation {
    /**
     * Start line of breakpoint location.
     */
    pub line: i32,

    /**
     * Optional start column of breakpoint location.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /**
     * Optional end line of breakpoint location if the location covers a range.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endLine: Option<i32>,

    /**
     * Optional end column of breakpoint location if the location covers a range.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endColumn: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceBreakpoint {
    /**
     * The source line of the breakpoint or logpoint.
     */
    pub line: i32,

    /**
     * An optional source column of the breakpoint.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /**
     * An optional expression for conditional breakpoints.
     * It is only honored by a debug adapter if the capability
     * 'supportsConditionalBreakpoints' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /**
     * An optional expression that controls how many hits of the breakpoint are
     * ignored.
     * The backend is expected to interpret the expression as needed.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsHitConditionalBreakpoints' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hitCondition: Option<String>,

    /**
     * If this attribute exists and is non-empty, the backend must not 'break'
     * (stop)
     * but log the message instead. Expressions within {} are interpolated.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsLogPoints' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logMessage: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FunctionBreakpoint {
    /**
     * The name of the function.
     */
    pub name: String,

    /**
     * An optional expression for conditional breakpoints.
     * It is only honored by a debug adapter if the capability
     * 'supportsConditionalBreakpoints' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /**
     * An optional expression that controls how many hits of the breakpoint are
     * ignored.
     * The backend is expected to interpret the expression as needed.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsHitConditionalBreakpoints' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hitCondition: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum DataBreakpointAccessType {
    Read,
    Write,
    ReadWrite,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DataBreakpoint {
    /**
     * An id representing the data. This id is returned from the
     * dataBreakpointInfo request.
     */
    pub dataId: String,

    /**
     * The access type of the data.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessType: Option<DataBreakpointAccessType>,

    /**
     * An optional expression for conditional breakpoints.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /**
     * An optional expression that controls how many hits of the breakpoint are
     * ignored.
     * The backend is expected to interpret the expression as needed.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hitCondition: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstructionBreakpoint {
    /**
     * The instruction reference of the breakpoint.
     * This should be a memory or instruction pointer reference from an
     * EvaluateResponse, Variable, StackFrame, GotoTarget, or Breakpoint.
     */
    pub instructionReference: String,

    /**
     * An optional offset from the instruction reference.
     * This can be negative.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,

    /**
     * An optional expression for conditional breakpoints.
     * It is only honored by a debug adapter if the capability
     * 'supportsConditionalBreakpoints' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,

    /**
     * An optional expression that controls how many hits of the breakpoint are
     * ignored.
     * The backend is expected to interpret the expression as needed.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsHitConditionalBreakpoints' is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hitCondition: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Breakpoint {
    /**
     * An optional identifier for the breakpoint. It is needed if breakpoint
     * events are used to update or remove breakpoints.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    /**
     * If true breakpoint could be set (but not necessarily at the desired
     * location).
     */
    pub verified: bool,

    /**
     * An optional message about the state of the breakpoint.
     * This is shown to the user and can be used to explain why a breakpoint could
     * not be verified.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /**
     * The source where the breakpoint is located.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,

    /**
     * The start line of the actual range covered by the breakpoint.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,

    /**
     * An optional start column of the actual range covered by the breakpoint.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /**
     * An optional end line of the actual range covered by the breakpoint.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endLine: Option<i32>,

    /**
     * An optional end column of the actual range covered by the breakpoint.
     * If no end line is given, then the end column is assumed to be in the start
     * line.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endColumn: Option<i32>,

    /**
     * An optional memory reference to where the breakpoint is set.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructionReference: Option<String>,

    /**
     * An optional offset from the instruction reference.
     * This can be negative.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum SteppingGranularity {
    Statement,
    Line,
    Instruction,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StepInTarget {
    /**
     * Unique identifier for a stepIn target.
     */
    pub id: i32,

    /**
     * The name of the stepIn target (shown in the UI).
     */
    pub label: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GotoTarget {
    /**
     * Unique identifier for a goto target. This is used in the goto request.
     */
    pub id: i32,

    /**
     * The name of the goto target (shown in the UI).
     */
    pub label: String,

    /**
     * The line of the goto target.
     */
    pub line: i32,

    /**
     * An optional column of the goto target.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /**
     * An optional end line of the range covered by the goto target.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endLine: Option<i32>,

    /**
     * An optional end column of the range covered by the goto target.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endColumn: Option<i32>,

    /**
     * Optional memory reference for the instruction pointer value represented by
     * this target.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructionPointerReference: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CompletionItem {
    /**
     * The label of this completion item. By default this is also the text that is
     * inserted when selecting this completion.
     */
    pub label: String,

    /**
     * If text is not falsy then it is inserted instead of the label.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /**
     * A String that should be used when comparing this item with other items.
     * When `falsy` the label is used.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortText: Option<String>,

    /**
     * A human-readable String with additional information about this item, like
     * type or symbol information.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,

    /**
     * The item's type. Typically the client uses this information to render the
     * item in the UI with an icon.
     */
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<CompletionItemType>,

    /**
     * This value determines the location (in the CompletionsRequest's 'text'
     * attribute) where the completion text is added.
     * If missing the text is added at the location specified by the
     * CompletionsRequest's 'column' attribute.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,

    /**
     * This value determines how many characters are overwritten by the completion
     * text.
     * If missing the value 0 is assumed which results in the completion text
     * being inserted.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,

    /**
     * Determines the start of the new selection after the text has been inserted
     * (or replaced).
     * The start position must in the range 0 and length of the completion text.
     * If omitted the selection starts at the end of the completion text.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectionStart: Option<i32>,

    /**
     * Determines the length of the new selection after the text has been inserted
     * (or replaced).
     * The selection can not extend beyond the bounds of the completion text.
     * If omitted the length is assumed to be 0.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectionLength: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum CompletionItemType {
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Class,
    Interface,
    Module,
    Property,
    Unit,
    Value,
    Enum,
    Keyword,
    Snippet,
    Text,
    Color,
    File,
    Reference,
    Customcolor,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub enum ChecksumAlgorithm {
    MD5,
    SHA1,
    SHA256,
    #[serde(rename = "timestamp")]
    Timestamp,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Checksum {
    /**
     * The algorithm used to calculate this checksum.
     */
    pub algorithm: ChecksumAlgorithm,

    /**
     * Value of the checksum.
     */
    pub checksum: String,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValueFormat {
    /**
     * Display the value in hex.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StackFrameFormat {
    /**
     * Display the value in hex.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hex: Option<bool>,

    /**
     * Displays parameters for the stack frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<bool>,

    /**
     * Displays the types of parameters for the stack frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameterTypes: Option<bool>,

    /**
     * Displays the names of parameters for the stack frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameterNames: Option<bool>,

    /**
     * Displays the values of parameters for the stack frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameterValues: Option<bool>,

    /**
     * Displays the line number of the stack frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<bool>,

    /**
     * Displays the module of the stack frame.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module: Option<bool>,

    /**
     * Includes all stack frames, including those the debug adapter might
     * otherwise hide.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeAll: Option<bool>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionFilterOptions {
    /**
     * ID of an exception filter returned by the 'exceptionBreakpointFilters'
     * capability.
     */
    pub filterId: String,

    /**
     * An optional expression for conditional exceptions.
     * The exception will break into the debugger if the result of the condition
     * is true.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionOptions {
    /**
     * A path that selects a single or multiple exceptions in a tree. If 'path' is
     * missing, the whole tree is selected.
     * By convention the first segment of the path is a category that is used to
     * group exceptions in the UI.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<ExceptionPathSegment>>,

    /**
     * Condition when a thrown exception should result in a break.
     */
    pub breakMode: ExceptionBreakMode,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ExceptionBreakMode {
    Never,
    Always,
    Unhandled,
    UserUnhandled,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionPathSegment {
    /**
     * If false or missing this segment matches the names provided, otherwise it
     * matches anything except the names provided.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,

    /**
     * Depending on the value of 'negate' the names that should match or not
     * match.
     */
    pub names: Vec<String>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
    /**
     * Message contained in the exception.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /**
     * Short type name of the exception object.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typeName: Option<String>,

    /**
     * Fully-qualified type name of the exception object.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fullTypeName: Option<String>,

    /**
     * Optional expression that can be evaluated in the current scope to obtain
     * the exception object.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluateName: Option<String>,

    /**
     * Stack trace at the time the exception was thrown.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackTrace: Option<String>,

    /**
     * Details of the exception contained by this exception, if any.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub innerException: Option<Vec<ExceptionDetails>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembledInstruction {
    /**
     * The address of the instruction. Treated as a hex value if prefixed with
     * '0x', or as a decimal value otherwise.
     */
    pub address: String,

    /**
     * Optional raw bytes representing the instruction and its operands, in an
     * implementation-defined format.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructionBytes: Option<String>,

    /**
     * Text representing the instruction and its operands, in an
     * implementation-defined format.
     */
    pub instruction: String,

    /**
     * Name of the symbol that corresponds with the location of this instruction,
     * if any.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,

    /**
     * Source location that corresponds to this instruction, if any.
     * Should always be set (if available) on the first instruction returned,
     * but can be omitted afterwards if this instruction maps to the same source
     * file as the previous instruction.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Source>,

    /**
     * The line within the source location that corresponds to this instruction,
     * if any.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,

    /**
     * The column within the line that corresponds to this instruction, if any.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,

    /**
     * The end line of the range that corresponds to this instruction, if any.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endLine: Option<i32>,

    /**
     * The end column of the range that corresponds to this instruction, if any.
     */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endColumn: Option<i32>,
}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum InvalidatedAreas {
    All,
    Stacks,
    Threads,
    Variables,
}

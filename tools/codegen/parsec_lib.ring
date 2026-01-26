# parsec_lib.ring - Parsing library for .rf configuration files
#
# Used by: parsec.ring, gendoc.ring
#
# Author: Youssef Saeed (ysdragon)

# ==============================================================================
# Global Variables
# ==============================================================================

if not islocal("$cCrateName")
    $cCrateName = ""
ok
if not islocal("$cLibPrefix")
    $cLibPrefix = ""
ok

# ==============================================================================
# Meta Parsing
# ==============================================================================

# Parse a meta line "key: value" and set global variables
# Returns: [key, value] or ["", ""] if invalid
Func ParseMetaLine cLine
    cLine = trim(cLine)
    nPos = substr(cLine, ":")
    if nPos > 0
        cKey = trim(left(cLine, nPos - 1))
        cValue = trim(substr(cLine, nPos + 1))
        switch lower(cKey)
        on "crate_name"
            $cCrateName = cValue
        on "lib_prefix"
            $cLibPrefix = cValue
        off
        return [cKey, cValue]
    ok
    return ["", ""]

# ==============================================================================
# Function Signature Parsing
# ==============================================================================

# Parse function signature: "fn name(param: Type, ...) -> ReturnType"
# Returns: [name, [[param_name, param_type], ...], return_type]
Func ParseFuncSignature cLine
    cLine = trim(cLine)
    
    # Remove "fn " prefix
    if left(lower(cLine), 3) = "fn "
        cLine = trim(substr(cLine, 4))
    ok
    
    # Find function name (up to first '(')
    nParenPos = substr(cLine, "(")
    if nParenPos = 0
        return ["", [], ""]
    ok
    
    cFuncName = trim(left(cLine, nParenPos - 1))
    
    # Find parameters (between '(' and ')')
    cRest = substr(cLine, nParenPos + 1)
    nClosePos = substr(cRest, ")")
    if nClosePos = 0
        return [cFuncName, [], ""]
    ok
    
    cParams = trim(left(cRest, nClosePos - 1))
    aParams = ParseFuncParams(cParams)
    
    # Find return type (after "->")
    cRest = substr(cRest, nClosePos + 1)
    cReturnType = ""
    nArrowPos = substr(cRest, "->")
    if nArrowPos > 0
        cReturnType = trim(substr(cRest, nArrowPos + 2))
    ok
    
    return [cFuncName, aParams, cReturnType]

# Parse comma-separated parameters: "name: Type, name2: Type2"
# Also handles: &self, &mut self, self
# Also handles: simple names without types (from signature comments)
# Returns: [[name, type], ...]
Func ParseFuncParams cParams
    aResult = []
    if trim(cParams) = ""
        return aResult
    ok
    
    # Handle nested generics - replace commas inside <> with placeholder
    cParams = ProtectGenericCommas(cParams)
    
    aParts = split(cParams, ",")
    for cPart in aParts
        cPart = RestoreGenericCommas(trim(cPart))
        
        # Handle special self parameters (no colon)
        if cPart = "&self" or cPart = "&mut self" or cPart = "self" or
           cPart = "& self" or cPart = "& mut self"
            aResult + [cPart, ""]
            loop
        ok
        
        nColonPos = substr(cPart, ":")
        if nColonPos > 0
            # Rust-style: name: type
            cName = trim(left(cPart, nColonPos - 1))
            cType = trim(substr(cPart, nColonPos + 1))
            aResult + [cName, cType]
        else
            # Simple style: just the name (from signature comments)
            # Handle optional params like "[, pretty]" or "[pretty]"
            lOptional = (left(cPart, 1) = "[")
            if lOptional
                cPart = substr(cPart, "[", "")
                cPart = substr(cPart, "]", "")
            ok
            cPart = trim(cPart)
            if cPart != ""
                if lOptional
                    aResult + ["[" + cPart + "]", ""]
                else
                    aResult + [cPart, ""]
                ok
            ok
        ok
    next
    
    return aResult

# Protect commas inside <> for generic types
Func ProtectGenericCommas cStr
    cResult = ""
    nDepth = 0
    for i = 1 to len(cStr)
        c = cStr[i]
        if c = "<"
            nDepth++
            cResult += c
        but c = ">"
            nDepth--
            cResult += c
        but c = "," and nDepth > 0
            cResult += "<<COMMA>>"
        else
            cResult += c
        ok
    next
    return cResult

# Restore protected commas
Func RestoreGenericCommas cStr
    return substr(cStr, "<<COMMA>>", ",")

# ==============================================================================
# Struct Parsing
# ==============================================================================

# Parse struct definition
# Returns: [name, [[field_name, field_type], ...]]
# Note: parsec.ring extends this with attributes
Func ParseStructDef cStructData
    aLines = str2list(cStructData)
    cStructName = ""
    aFields = []
    lInFields = false
    
    for cLine in aLines
        cLine = trim(cLine)
        if cLine = "" loop ok
        
        # Check for struct name line
        if substr(lower(cLine), "struct ") or (cStructName = "" and not lInFields)
            cLine = substr(cLine, "struct ", "")
            cLine = trim(cLine)
            nBracePos = substr(cLine, "{")
            if nBracePos > 0
                cStructName = trim(left(cLine, nBracePos - 1))
                lInFields = true
            else
                cStructName = trim(cLine)
            ok
            loop
        ok
        
        # Opening brace
        if cLine = "{"
            lInFields = true
            loop
        ok
        
        # Closing brace
        if cLine = "}" or left(cLine, 1) = "}"
            lInFields = false
            loop
        ok
        
        # Skip attributes
        if left(cLine, 1) = "#"
            loop
        ok
        
        # Parse field: name: Type or pub name: Type
        if lInFields
            if left(lower(cLine), 4) = "pub "
                cLine = trim(substr(cLine, 5))
            ok
            # Remove trailing comma
            if right(cLine, 1) = ","
                cLine = left(cLine, len(cLine) - 1)
            ok
            nColonPos = substr(cLine, ":")
            if nColonPos > 0
                cFieldName = trim(left(cLine, nColonPos - 1))
                cFieldType = trim(substr(cLine, nColonPos + 1))
                aFields + [cFieldName, cFieldType]
            ok
        ok
    next
    
    return [cStructName, aFields]

# ==============================================================================
# Impl Parsing
# ==============================================================================

# Parse impl block
# Returns: [struct_name, [[method_name, params, return_type], ...]]
# Note: parsec.ring extends this with is_static flag
Func ParseImplDef cImplData
    aLines = str2list(cImplData)
    cStructName = ""
    aMethods = []
    
    for cLine in aLines
        cLine = trim(cLine)
        if cLine = "" loop ok
        
        # Check for impl line
        if left(lower(cLine), 5) = "impl "
            cRest = trim(substr(cLine, 6))
            if right(cRest, 1) = "{"
                cRest = trim(left(cRest, len(cRest) - 1))
            ok
            cStructName = cRest
            loop
        ok
        
        # Parse method: pub fn name(...) -> Type
        if substr(lower(cLine), "fn ")
            if left(lower(cLine), 4) = "pub "
                cLine = trim(substr(cLine, 5))
            ok
            
            aFunc = ParseFuncSignature(cLine)
            if aFunc[1] != ""
                aMethods + [aFunc[1], aFunc[2], aFunc[3]]
            ok
        ok
    next
    
    return [cStructName, aMethods]

# ==============================================================================
# Type Formatting (for documentation)
# ==============================================================================

# Format Rust type for Ring documentation
Func FormatRustType cType
    cType = trim(cType)
    
    switch cType
    on "String"
        return "String"
    on "&str"
        return "String"
    on "i32"
        return "Number"
    on "i64"
        return "Number"
    on "f64"
        return "Number"
    on "f32"
        return "Number"
    on "u32"
        return "Number"
    on "u64"
        return "Number"
    on "bool"
        return "Boolean"
    on "()"
        return ""
    off
    
    if left(cType, 7) = "Result<"
        return cType
    ok
    if left(cType, 7) = "Option<"
        return cType
    ok
    
    return cType

# Format parameter list for display
Func FormatParamList aParams
    if len(aParams) = 0
        return ""
    ok
    
    cResult = ""
    for i = 1 to len(aParams)
        if i > 1
            cResult += ", "
        ok
        cResult += aParams[i][1]
    next
    return cResult

# ==============================================================================
# Constant Parsing
# ==============================================================================

# Parse constant: "NAME: Type" or "NAME: Type = value"
# Returns: [name, type, value]
Func ParseConstantDef cLine
    cLine = trim(cLine)
    
    # Check for = sign
    nEqPos = substr(cLine, "=")
    cValue = ""
    if nEqPos > 0
        cValue = trim(substr(cLine, nEqPos + 1))
        cLine = trim(left(cLine, nEqPos - 1))
    ok
    
    # Check for : type
    nColonPos = substr(cLine, ":")
    cType = ""
    cName = cLine
    if nColonPos > 0
        cName = trim(left(cLine, nColonPos - 1))
        cType = trim(substr(cLine, nColonPos + 1))
    ok
    
    return [cName, cType, cValue]

# ==============================================================================
# Register Parsing
# ==============================================================================

# Parse register line: "name" or "ring_name => rust_func_name"
# Returns: [ring_name, rust_func_name]
Func ParseRegisterLine cLine
    cLine = trim(cLine)
    if cLine = "" return ["", ""] ok
    
    nArrowPos = substr(cLine, "=>")
    if nArrowPos > 0
        cRingName = trim(left(cLine, nArrowPos - 1))
        cRustName = trim(substr(cLine, nArrowPos + 2))
        return [cRingName, cRustName]
    ok
    
    # Auto-prefix: "encode" becomes "prefix_encode" => "ring_prefix_encode"
    cPrefix = $cLibPrefix
    if cPrefix != ""
        cPrefix += "_"
    ok
    cRingName = cPrefix + cLine
    cRustName = "ring_" + cPrefix + cLine
    return [cRingName, cRustName]

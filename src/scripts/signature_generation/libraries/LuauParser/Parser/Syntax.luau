--!strict

export type Position = {
	line: number,
	column: number,
}

export type Location = {
	begin: Position,
	end_: Position,
}


export type AstName = {
	value: string,
}

export type AstExpr = AstExprGroup
| AstExprConstantNil
| AstExprConstantBool
| AstExprConstantNumber
| AstExprConstantInteger
| AstExprConstantString
| AstExprLocal
| AstExprGlobal
| AstExprVarargs
| AstExprCall
| AstExprIndexName
| AstExprIndexExpr
| AstExprFunction
| AstExprTable
| AstExprUnary
| AstExprBinary
| AstExprTypeAssertion
| AstExprIfElse
| AstExprInterpString
| AstExprInstantiate
| AstExprError

export type AstStat = AstStatBlock
| AstStatIf
| AstStatWhile
| AstStatRepeat
| AstStatBreak
| AstStatContinue
| AstStatReturn
| AstStatExpr
| AstStatLocal
| AstStatFor
| AstStatForIn
| AstStatAssign
| AstStatCompoundAssign
| AstStatFunction
| AstStatLocalFunction
| AstStatTypeAlias
| AstStatTypeFunction
| AstStatError

export type AstTypePack = AstTypePackExplicit | AstTypePackVariadic | AstTypePackGeneric

export type AstType = AstTypeReference
| AstTypeTable
| AstTypeFunction
| AstTypeTypeof
| AstTypeUnion
| AstTypeIntersection
| AstTypeSingletonBool
| AstTypeSingletonString
| AstTypeGroup
| AstTypeError
| AstTypeOptional

export type AstTypeList = {
	types: { AstType },
	tailType: AstTypePack?,
}

export type AstTypeOrPack = {
	type: AstType?,
	typePack: AstTypePack?,
}

export type AstArgumentName = {
	name: string,
	location: Location,
}

export type AstLocal = {
	name: string,
	location: Location,
	shadow: AstLocal?,
	functionDepth: number,
	loopDepth: number,
	annotation: AstType?,
	isConst: boolean?,
}

export type AstAttr = {
	kind: "Attr",
	location: Location,
	type: string?,
	args: { AstExpr },
	name: string?,
}

export type AstGenericType = {
	cstNode: CstGenericType?,
	kind: "GenericType",
	location: Location,
	name: string,
	defaultValue: AstType?,
}

export type AstGenericTypePack = {
	cstNode: CstGenericTypePack?,
	kind: "GenericTypePack",
	location: Location,
	name: string,
	defaultValue: AstTypePack?,
}

export type AstExprGroup = {
	kind: "ExprGroup",
	location: Location,
	expr: AstExpr,
}

export type AstExprConstantNil = {
	kind: "ExprConstantNil",
	location: Location,
}

export type AstExprConstantBool = {
	kind: "ExprConstantBool",
	location: Location,
	value: boolean,
}

export type AstExprConstantNumber = {
	cstNode: CstExprConstantNumber?,
	kind: "ExprConstantNumber",
	location: Location,
	value: number,
	parseResult: "Ok" | "Imprecise" | "HexOverflow" | "BinOverflow" | "IntOverflow" | "Malformed",
}

export type AstExprConstantInteger = {
	cstNode: CstExprConstantInteger?,
	kind: "ExprConstantInteger",
	location: Location,
	low: number,
	high: number,
	parseResult: "Ok" | "Imprecise" | "HexOverflow" | "BinOverflow" | "IntOverflow" | "Malformed",
}

export type AstExprConstantString = {
	cstNode: CstExprConstantString?,
	kind: "ExprConstantString",
	location: Location,
	value: string,
	quoteStyle: number,
}

export type AstExprLocal = {
	kind: "ExprLocal",
	location: Location,
	["local"]: AstLocal,
	upvalue: boolean,
}

export type AstExprGlobal = {
	kind: "ExprGlobal",
	location: Location,
	name: string,
}

export type AstExprVarargs = {
	kind: "ExprVarargs",
	location: Location,
}

export type AstExprCall = {
	cstNode: CstExprCall?,
	kind: "ExprCall",
	location: Location,
	func: AstExpr,
	args: { AstExpr },
	self: boolean,
	typeArguments: { AstTypeOrPack },
	argLocation: Location,
}

export type AstExprIndexName = {
	kind: "ExprIndexName",
	location: Location,
	expr: AstExpr,
	index: string,
	indexLocation: Location,
	opPosition: Position,
	op: number,
}

export type AstExprIndexExpr = {
	cstNode: CstExprIndexExpr?,
	kind: "ExprIndexExpr",
	location: Location,
	expr: AstExpr,
	index: AstExpr,
}

export type AstExprFunction = {
	cstNode: CstExprFunction?,
	kind: "ExprFunction",
	location: Location,
	attributes: { AstAttr },
	generics: { AstGenericType },
	genericPacks: { AstGenericTypePack },
	self: AstLocal?,
	args: { AstLocal },
	returnAnnotation: AstTypePack?,
	vararg: boolean,
	varargLocation: Location?,
	varargAnnotation: AstTypePack?,
	body: AstStatBlock,
	functionDepth: number,
	debugname: string?,
	argLocation: Location?,
}

export type AstExprTableItem = {
	kind: "List" | "Record" | "General",
	key: AstExpr?,
	value: AstExpr,
}

export type AstExprTable = {
	cstNode: CstExprTable?,
	kind: "ExprTable",
	location: Location,
	items: { AstExprTableItem },
}

export type AstExprUnary = {
	cstNode: CstExprOp?,
	kind: "ExprUnary",
	location: Location,
	op: number,
	expr: AstExpr,
}

export type AstExprBinary = {
	cstNode: CstExprOp?,
	kind: "ExprBinary",
	location: Location,
	op: number,
	left: AstExpr,
	right: AstExpr,
}

export type AstExprTypeAssertion = {
	cstNode: CstExprTypeAssertion?,
	kind: "ExprTypeAssertion",
	location: Location,
	expr: AstExpr,
	annotation: AstType,
}

export type AstExprInstantiate = {
	cstNode: CstExprExplicitTypeInstantiation?,
	kind: "ExprInstantiate",
	location: Location,
	expr: AstExpr,
	typeArguments: { AstTypeOrPack },
}

export type AstExprIfElse = {
	cstNode: CstExprIfElse?,
	kind: "ExprIfElse",
	location: Location,
	condition: AstExpr,
	hasThen: boolean,
	trueExpr: AstExpr,
	hasElse: boolean,
	falseExpr: AstExpr,
}

export type AstExprInterpString = {
	cstNode: CstExprInterpString?,
	kind: "ExprInterpString",
	location: Location,
	strings: { string },
	expressions: { AstExpr },
}

export type AstExprError = {
	kind: "ExprError",
	location: Location,
	expressions: { AstExpr },
	messageIndex: number,
}

export type AstStatBlock = {
	cstNode: CstStatDo | CstStatDo_DEPRECATED | nil,
	kind: "StatBlock",
	location: Location,
	body: { AstStat },
	hasEnd: boolean,
	hasSemicolon: boolean?,
}

export type AstStatIf = {
	kind: "StatIf",
	location: Location,
	condition: AstExpr,
	thenbody: AstStatBlock,
	elsebody: AstStat?,
	thenLocation: Location?,
	elseLocation: Location?,
	hasSemicolon: boolean?,
}

export type AstStatWhile = {
	kind: "StatWhile",
	location: Location,
	condition: AstExpr,
	body: AstStatBlock,
	hasDo: boolean,
	doLocation: Location,
	hasSemicolon: boolean?,
}

export type AstStatRepeat = {
	cstNode: CstStatRepeat?,
	kind: "StatRepeat",
	location: Location,
	condition: AstExpr,
	body: AstStatBlock,
	hasUntil: boolean,
	hasSemicolon: boolean?,
}

export type AstStatBreak = {
	kind: "StatBreak",
	location: Location,
	hasSemicolon: boolean?
}

export type AstStatContinue = {
	kind: "StatContinue",
	location: Location,
	hasSemicolon: boolean?
}

export type AstStatReturn = {
	cstNode: CstStatReturn?,
	kind: "StatReturn",
	location: Location,
	list: { AstExpr },
	hasSemicolon: boolean?,
}
export type AstStatExpr = {
	kind: "StatExpr",
	location: Location,
	expr: AstExpr,
	hasSemicolon: boolean?
}

export type AstStatLocal = {
	cstNode: CstStatLocal?,
	kind: "StatLocal",
	location: Location,
	vars: { AstLocal },
	values: { AstExpr },
	equalsSignLocation: Location?,
	hasSemicolon: boolean?,
}

export type AstStatFor = {
	cstNode: CstStatFor?,
	kind: "StatFor",
	location: Location,
	var: AstLocal,
	from: AstExpr,
	to: AstExpr,
	step: AstExpr?,
	body: AstStatBlock,
	hasDo: boolean,
	doLocation: Location,
	hasSemicolon: boolean?,
}

export type AstStatForIn = {
	cstNode: CstStatForIn?,
	kind: "StatForIn",
	location: Location,
	vars: { AstLocal },
	values: { AstExpr },
	body: AstStatBlock,
	hasIn: boolean,
	inLocation: Location,
	hasDo: boolean,
	doLocation: Location,
	hasSemicolon: boolean?,
}

export type AstStatAssign = {
	cstNode: CstStatAssign?,
	kind: "StatAssign",
	location: Location,
	vars: { AstExpr },
	values: { AstExpr },
	hasSemicolon: boolean?,
}

export type AstStatCompoundAssign = {
	cstNode: CstStatCompoundAssign?,
	kind: "StatCompoundAssign",
	location: Location,
	op: number,
	var: AstExpr,
	value: AstExpr,
	hasSemicolon: boolean?,
}

export type AstStatFunction = {
	cstNode: CstStatFunction?,
	kind: "StatFunction",
	location: Location,
	name: AstExpr,
	func: AstExprFunction,
	hasSemicolon: boolean?,
}

export type AstStatLocalFunction = {
	cstNode: CstStatLocalFunction?,
	kind: "StatLocalFunction",
	location: Location,
	name: AstLocal?,
	func: AstExprFunction,
	hasSemicolon: boolean?,
}

export type AstStatTypeAlias = {
	cstNode: CstStatTypeAlias?,
	kind: "StatTypeAlias",
	location: Location,
	name: string,
	nameLocation: Location,
	generics: { AstGenericType },
	genericPacks: { AstGenericTypePack },
	type: AstType,
	exported: boolean,
	hasSemicolon: boolean?,
}

export type AstStatTypeFunction = {
	cstNode: CstStatTypeFunction?,
	kind: "StatTypeFunction",
	location: Location,
	name: string,
	nameLocation: Location,
	body: AstExprFunction,
	exported: boolean,
	hasErrors: boolean,
	hasSemicolon: boolean?,
}

export type AstTableProp = {
	kind: "TableProp",
	name: AstName,
	location: Location,
	type: AstType,
	access: string,
	accessLocation: Location?,
}

export type AstTableIndexer = {
	kind: "TableIndexer",
	indexType: AstType,
	resultType: AstType,
	location: Location,
	access: string,
	accessLocation: Location?,
}

export type AstStatError = {
	kind: "StatError",
	location: Location,
	expressions: { AstExpr },
	statements: { AstStat },
	messageIndex: number,
	hasSemicolon: boolean?,
}

export type AstTypeReference = {
	cstNode: CstTypeReference?,
	kind: "TypeReference",
	location: Location,
	hasParameterList: boolean,
	prefix: string?,
	prefixLocation: Location?,
	name: string,
	nameLocation: Location,
	parameters: { AstTypeOrPack },
}

export type AstTypeTable = {
	cstNode: CstTypeTable?,
	kind: "TypeTable",
	location: Location,
	props: { AstTableProp },
	indexer: AstTableIndexer?,
}

export type AstTypeFunction = {
	cstNode: CstTypeFunction?,
	kind: "TypeFunction",
	location: Location,
	attributes: { AstAttr },
	generics: { AstGenericType },
	genericPacks: { AstGenericTypePack },
	argTypes: AstTypeList,
	argNames: { AstArgumentName | boolean },
	returnTypes: AstTypePack,
}

export type AstTypeTypeof = {
	cstNode: CstTypeTypeof?,
	kind: "TypeTypeof",
	location: Location,
	expr: AstExpr,
}

export type AstTypeUnion = {
	cstNode: CstTypeUnion?,
	kind: "TypeUnion",
	location: Location,
	types: { AstType },
}

export type AstTypeIntersection = {
	cstNode: CstTypeIntersection?,
	kind: "TypeIntersection",
	location: Location,
	types: { AstType },
}

export type AstTypeSingletonBool = {
	kind: "TypeSingletonBool",
	location: Location,
	value: boolean,
}

export type AstTypeSingletonString = {
	kind: "TypeSingletonString",
	location: Location,
	value: string,
}

export type AstTypeGroup = {
	kind: "TypeGroup",
	location: Location,
	type: AstType,
}

export type AstTypeError = {
	kind: "TypeError",
	location: Location,
	types: { AstType },
	isMissing: boolean,
	messageIndex: number,
}

export type AstTypeOptional = {
	kind: "TypeOptional",
	location: Location,
}

export type AstTypePackExplicit = {
	cstNode: CstTypePackExplicit?,
	kind: "TypePackExplicit",
	location: Location,
	types: { AstType },
	tailType: AstTypePack?,
}

export type AstTypePackVariadic = {
	kind: "TypePackVariadic",
	location: Location,
	variadicType: AstType,
}

export type AstTypePackGeneric = {
	cstNode: CstTypePackGeneric?,
	kind: "TypePackGeneric",
	location: Location,
	genericName: string,
}

export type Binding = {
	name: AstName,
	location: Location,
	annotation: AstType?,
	colonPosition: Position?,
	isConst: boolean?,
}

export type BindingList = { Binding }
export type Attrs = { AstAttr }

export type CstExprConstantNumber = {
	kind: "CstExprConstantNumber",
	value: string?,
}

export type CstExprConstantInteger = {
	kind: "CstExprConstantInteger",
	value: string?,
}

export type CstExprConstantString = {
	kind: "CstExprConstantString",
	sourceString: string?,
	quoteStyle: number,
	blockDepth: number,
}

export type CstTypeInstantiation = {
	kind: "CstTypeInstantiation",
	leftArrow1Position: Position,
	leftArrow2Position: Position?,
	commaPositions: { Position },
	rightArrow1Position: Position?,
	rightArrow2Position: Position,
}

export type CstExprCall = {
	kind: "CstExprCall",
	openParens: Position?,
	closeParens: Position?,
	commaPositions: { Position },
	explicitTypes: CstTypeInstantiation?,
}

export type CstExprIndexExpr = {
	kind: "CstExprIndexExpr",
	openBracketPosition: Position,
	closeBracketPosition: Position,
}

export type CstExprFunction = {
	kind: "CstExprFunction",
	functionKeywordPosition: Position,
	openGenericsPosition: Position,
	genericsCommaPositions: { Position },
	closeGenericsPosition: Position,
	argsAnnotationColonPositions: { Position? },
	argsCommaPositions: { Position },
	varargAnnotationColonPosition: Position?,
	returnSpecifierPosition: Position,
}

export type CstExprTableItem = {
	kind: "Record" | "General" | "List",
	equalsPosition: Position?,
	separator: number?,
	separatorPosition: Position,
	indexerOpenPosition: Position?,
	indexerClosePosition: Position?,
}

export type CstExprTable = {
	kind: "CstExprTable",
	items: { CstExprTableItem },
}

export type CstExprOp = {
	kind: "CstExprOp",
	opPosition: Position,
}

export type CstExprTypeAssertion = {
	kind: "CstExprTypeAssertion",
	opPosition: Position,
}

export type CstExprIfElse = {
	kind: "CstExprIfElse",
	thenPosition: Position,
	elsePosition: Position,
	isElseIf: boolean,
}

export type CstExprInterpString = {
	kind: "CstExprInterpString",
	sourceStrings: { string },
	stringPositions: { Position },
}

export type CstExprExplicitTypeInstantiation = {
	kind: "CstExprExplicitTypeInstantiation",
	instantiation: CstTypeInstantiation,
}

export type CstStatDo = {
	kind: "CstStatDo",
	statsStart: Position,
	endPosition: Position,
}

export type CstStatDo_DEPRECATED = {
	kind: "CstStatDo_DEPRECATED",
	endPosition: Position,
}

export type CstStatRepeat = {
	kind: "CstStatRepeat",
	untilPosition: Position,
}

export type CstStatReturn = {
	kind: "CstStatReturn",
	commaPositions: { Position },
}

export type CstStatLocal = {
	kind: "CstStatLocal",
	varsAnnotationColonPositions: { Position? },
	varsCommaPositions: { Position },
	valuesCommaPositions: { Position },
}

export type CstStatFor = {
	kind: "CstStatFor",
	annotationColonPosition: Position?,
	equalsPosition: Position,
	endCommaPosition: Position,
	stepCommaPosition: Position?,
}

export type CstStatForIn = {
	kind: "CstStatForIn",
	varsAnnotationColonPositions: { Position? },
	varsCommaPositions: { Position },
	valuesCommaPositions: { Position },
}

export type CstStatAssign = {
	kind: "CstStatAssign",
	varsCommaPositions: { Position },
	equalsPosition: Position,
	valuesCommaPositions: { Position },
}

export type CstStatCompoundAssign = {
	kind: "CstStatCompoundAssign",
	opPosition: Position,
}

export type CstStatFunction = {
	kind: "CstStatFunction",
	functionKeywordPosition: Position,
}

export type CstStatLocalFunction = {
	kind: "CstStatLocalFunction",
	localKeywordPosition: Position,
	functionKeywordPosition: Position,
}

export type CstGenericType = {
	kind: "CstGenericType",
	defaultEqualsPosition: Position?,
}

export type CstGenericTypePack = {
	kind: "CstGenericTypePack",
	ellipsisPosition: Position,
	defaultEqualsPosition: Position?,
}

export type CstStatTypeAlias = {
	kind: "CstStatTypeAlias",
	typeKeywordPosition: Position,
	genericsOpenPosition: Position?,
	genericsCommaPositions: { Position },
	genericsClosePosition: Position?,
	equalsPosition: Position,
}

export type CstStatTypeFunction = {
	kind: "CstStatTypeFunction",
	typeKeywordPosition: Position,
	functionKeywordPosition: Position,
}

export type CstTypeReference = {
	kind: "CstTypeReference",
	prefixPointPosition: Position?,
	openParametersPosition: Position?,
	parametersCommaPositions: { Position },
	closeParametersPosition: Position?,
}

export type CstTypeTableItem = {
	kind: "Property" | "StringProperty" | "Indexer",
	colonPosition: Position?,
	separator: number?,
	separatorPosition: Position?,
	indexerOpenPosition: Position?,
	indexerClosePosition: Position?,
	stringInfo: CstExprConstantString?,
	stringPosition: Position?,
}

export type CstTypeTable = {
	kind: "CstTypeTable",
	items: { CstTypeTableItem },
	isArray: boolean,
}

export type CstTypeFunction = {
	kind: "CstTypeFunction",
	openGenericsPosition: Position,
	genericsCommaPositions: { Position },
	closeGenericsPosition: Position,
	openArgsPosition: Position,
	argumentNameColonPositions: { Position | boolean },
	argumentsCommaPositions: { Position },
	closeArgsPosition: Position,
	returnArrowPosition: Position,
}

export type CstTypeTypeof = {
	kind: "CstTypeTypeof",
	openPosition: Position,
	closePosition: Position,
}

export type CstTypeUnion = {
	kind: "CstTypeUnion",
	leadingPosition: Position?,
	separatorPositions: { Position },
}

export type CstTypeIntersection = {
	kind: "CstTypeIntersection",
	leadingPosition: Position?,
	separatorPositions: { Position },
}

export type CstTypePackExplicit = {
	kind: "CstTypePackExplicit",
	openParenthesesPosition: Position?,
	closeParenthesesPosition: Position?,
	commaPositions: { Position }?,
}

export type CstTypePackGeneric = {
	kind: "CstTypePackGeneric",
	ellipsisPosition: Position,
}

export type AstNode = AstExpr
| AstStat
| AstType
| AstTypePack
| AstGenericType
| AstGenericTypePack
| AstAttr
| AstLocal

export type ParseError = {
	location: Location,
	message: string,
}

export type Options = {
	allowDeclarationSyntax: boolean?,
	captureComments: boolean?,
	parseFragment: {
		localMap: { [string]: AstLocal? },
		localStack: { AstLocal? },
		resumePosition: Position,
	}?,
	storeCstData: boolean?,
}

export type Lexeme = {
	type: number,
	location: Location,
}

export type Comment = {
	type: number,
	location: Location,
}

export type HotComment = {
	header: boolean,
	content: string,
	location: Location,
}

export type FunctionState = {
	vararg: boolean,
	loopDepth: number,
}

export type Result = {
	root: AstStatBlock?,
	commentLocations: { Comment },
	hotcomments: { HotComment },
	errors: { ParseError },
}

return {}

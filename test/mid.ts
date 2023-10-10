const DEFINE_SEP = [
  "semi:;",
  "comma:,",
  "lparen:(",
  "rparen:)",
  "lbrack:[",
  "rbrack:]",
  "lcurly:{",
  "rcurly:}",
  "label:::",
  "colon::",
];

const DEFINE_OP = [
  "add:+",
  "minus:-",
  "mul:*",
  "pow:^",
  "mod:%",
  "bor:|",
  "band:&",
  "len:#",
  "idiv://",
  "div:/",
  "ne:~=",
  "wave:~",
  "noteq:!=",
  "not:!",
  "eq:==",
  "assign:=",
  "shl:<<",
  "le:<=",
  "lt:<",
  "shr:>>",
  "ge>=",
  "gt:>",
  "vararg:...",
  "concat:..",
  "dot:.",
  'string:"',
];

const DEFINE_KEYWORD = [
  "and",
  "break",
  "do",
  "if",
  "else",
  "true",
  "false",
  "return",
  "function",
  "for",
  "in",
  "case",
  "continue",
  "const",
];

const DEFINE_EBNF = [
  ";",
  "break",
  "if exp then block ",
  "functioncall",
  "function funcname funcbody",
];

let defineToken = DEFINE_SEP.map((it) => {
  const [name, value] = it.split(":");
  return `const TOKEN_SEP_${name.toUpperCase()} = '${value}';`;
}).join("\n");

defineToken += DEFINE_OP.map((it) => {
  const [name, value] = it.split(":");
  return `const TOKEN_OP_${name.toUpperCase()} = '${value}'`;
}).join("\n");

defineToken += DEFINE_KEYWORD.map((it) =>
  `const TOKEN_KW_${it.toUpperCase()} = '${it}'`
).join("\n");

console.log(defineToken);

//! [Globals](https://github.com/sindresorhus/globals/blob/main/globals.json)
//! Each global is given a value of true or false.
//! A value of true indicates that the variable may be overwritten.
//! A value of false indicates that the variable should be considered read-only.

use phf::{phf_map, phf_set, Map};

pub const BUILTINS: Map<&'static str, bool> = phf_map! {
    "AggregateError" => false,
    "Array" => false,
    "ArrayBuffer" => false,
    "Atomics" => false,
    "BigInt" => false,
    "BigInt64Array" => false,
    "BigUint64Array" => false,
    "Boolean" => false,
    "constructor" => false,
    "DataView" => false,
    "Date" => false,
    "decodeURI" => false,
    "decodeURIComponent" => false,
    "encodeURI" => false,
    "encodeURIComponent" => false,
    "Error" => false,
    "escape" => false,
    "eval" => false,
    "EvalError" => false,
    "FinalizationRegistry" => false,
    "Float32Array" => false,
    "Float64Array" => false,
    "Function" => false,
    "globalThis" => false,
    "hasOwnProperty" => false,
    "Infinity" => false,
    "Int16Array" => false,
    "Int32Array" => false,
    "Int8Array" => false,
    "isFinite" => false,
    "isNaN" => false,
    "isPrototypeOf" => false,
    "JSON" => false,
    "Map" => false,
    "Math" => false,
    "NaN" => false,
    "Number" => false,
    "Object" => false,
    "parseFloat" => false,
    "parseInt" => false,
    "Promise" => false,
    "propertyIsEnumerable" => false,
    "Proxy" => false,
    "RangeError" => false,
    "ReferenceError" => false,
    "Reflect" => false,
    "RegExp" => false,
    "Set" => false,
    "SharedArrayBuffer" => false,
    "String" => false,
    "Symbol" => false,
    "Diagnostic" => false,
    "toLocaleString" => false,
    "toString" => false,
    "TypeError" => false,
    "Uint16Array" => false,
    "Uint32Array" => false,
    "Uint8Array" => false,
    "Uint8ClampedArray" => false,
    "undefined" => false,
    "unescape" => false,
    "URIError" => false,
    "valueOf" => false,
    "WeakMap" => false,
    "WeakRef" => false,
    "WeakSet" => false
};

pub const PRE_DEFINE_VAR: Map<&'static str, bool> = phf_map! {
    "undefined" => false,
    "Infinity" => false,
    "NaN" => false,
    "eval" => false,
    "arguments" => false,
};

pub const GLOBAL_OBJECT_NAMES: phf::Set<&'static str> = phf_set! {
    "global",
    "globalThis",
    "self",
    "window",
};

/// set of valid ARIA properties from the WAI-ARIA 1.1 specifications.
/// Reference: <https://www.w3.org/TR/wai-aria/#state_prop_def>
pub const VALID_ARIA_PROPS: phf::Set<&'static str> = phf_set! {
    "aria-activedescendant",
    "aria-atomic",
    "aria-autocomplete",
    "aria-busy",
    "aria-checked",
    "aria-colcount",
    "aria-colindex",
    "aria-colspan",
    "aria-controls",
    "aria-current",
    "aria-describedby",
    "aria-details",
    "aria-disabled",
    "aria-dropeffect",
    "aria-errormessage",
    "aria-expanded",
    "aria-flowto",
    "aria-grabbed",
    "aria-haspopup",
    "aria-hidden",
    "aria-invalid",
    "aria-keyshortcuts",
    "aria-label",
    "aria-labelledby",
    "aria-level",
    "aria-live",
    "aria-modal",
    "aria-multiline",
    "aria-multiselectable",
    "aria-orientation",
    "aria-owns",
    "aria-placeholder",
    "aria-posinset",
    "aria-pressed",
    "aria-readonly",
    "aria-relevant",
    "aria-required",
    "aria-roledescription",
    "aria-rowcount",
    "aria-rowindex",
    "aria-rowspan",
    "aria-selected",
    "aria-setsize",
    "aria-sort",
    "aria-valuemax",
    "aria-valuemin",
    "aria-valuenow",
    "aria-valuetext"
};

/// set of valid ARIA role definitions
/// Reference: <https://www.w3.org/TR/wai-aria/#role_definitions>
pub const VALID_ARIA_ROLES: phf::Set<&'static str> = phf_set! {
    "alert",
  "alertdialog",
  "application",
  "article",
  "banner",
  "blockquote",
  "button",
  "caption",
  "cell",
  "checkbox",
  "code",
  "columnheader",
  "combobox",
  "complementary",
  "contentinfo",
  "definition",
  "deletion",
  "dialog",
  "directory",
  "document",
  "emphasis",
  "feed",
  "figure",
  "form",
  "generic",
  "grid",
  "gridcell",
  "group",
  "heading",
  "img",
  "insertion",
  "link",
  "list",
  "listbox",
  "listitem",
  "log",
  "main",
  "mark",
  "marquee",
  "math",
  "menu",
  "menubar",
  "menuitem",
  "menuitemcheckbox",
  "menuitemradio",
  "meter",
  "navigation",
  "none",
  "note",
  "option",
  "paragraph",
  "presentation",
  "progressbar",
  "radio",
  "radiogroup",
  "region",
  "row",
  "rowgroup",
  "rowheader",
  "scrollbar",
  "search",
  "searchbox",
  "separator",
  "slider",
  "spinbutton",
  "status",
  "strong",
  "subscript",
  "superscript",
  "switch",
  "tab",
  "table",
  "tablist",
  "tabpanel",
  "term",
  "textbox",
  "time",
  "timer",
  "toolbar",
  "tooltip",
  "tree",
  "treegrid",
  "treeitem",
  "doc-abstract",
  "doc-acknowledgments",
  "doc-afterword",
  "doc-appendix",
  "doc-backlink",
  "doc-biblioentry",
  "doc-bibliography",
  "doc-biblioref",
  "doc-chapter",
  "doc-colophon",
  "doc-conclusion",
  "doc-cover",
  "doc-credit",
  "doc-credits",
  "doc-dedication",
  "doc-endnote",
  "doc-endnotes",
  "doc-epigraph",
  "doc-epilogue",
  "doc-errata",
  "doc-example",
  "doc-footnote",
  "doc-foreword",
  "doc-glossary",
  "doc-glossref",
  "doc-index",
  "doc-introduction",
  "doc-noteref",
  "doc-notice",
  "doc-pagebreak",
  "doc-pagelist",
  "doc-part",
  "doc-preface",
  "doc-prologue",
  "doc-pullquote",
  "doc-qna",
  "doc-subtitle",
  "doc-tip",
  "doc-toc",
  "graphics-document",
  "graphics-object",
  "graphics-symbol"
};

pub const HTML_TAG: phf::Set<&'static str> = phf_set! {
    "a",
    "abbr",
    "acronym",
    "address",
    "applet",
    "area",
    "article",
    "aside",
    "audio",
    "b",
    "base",
    "basefont",
    "bdi",
    "bdo",
    "bgsound",
    "big",
    "blink",
    "blockquote",
    "body",
    "br",
    "button",
    "canvas",
    "caption",
    "center",
    "cite",
    "code",
    "col",
    "colgroup",
    "command",
    "content",
    "data",
    "datalist",
    "dd",
    "del",
    "details",
    "dfn",
    "dialog",
    "dir",
    "div",
    "dl",
    "dt",
    "element",
    "em",
    "embed",
    "fieldset",
    "figcaption",
    "figure",
    "font",
    "footer",
    "form",
    "frame",
    "frameset",
    "h1",
    "h2",
    "h3",
    "h4",
    "h5",
    "h6",
    "head",
    "header",
    "hgroup",
    "hr",
    "html",
    "i",
    "iframe",
    "image",
    "img",
    "input",
    "ins",
    "isindex",
    "kbd",
    "keygen",
    "label",
    "legend",
    "li",
    "link",
    "listing",
    "main",
    "map",
    "mark",
    "marquee",
    "math",
    "menu",
    "menuitem",
    "meta",
    "meter",
    "multicol",
    "nav",
    "nextid",
    "nobr",
    "noembed",
    "noframes",
    "noscript",
    "object",
    "ol",
    "optgroup",
    "option",
    "output",
    "p",
    "param",
    "picture",
    "plaintext",
    "pre",
    "progress",
    "q",
    "rb",
    "rbc",
    "rp",
    "rt",
    "rtc",
    "ruby",
    "s",
    "samp",
    "script",
    "search",
    "section",
    "select",
    "shadow",
    "slot",
    "small",
    "source",
    "spacer",
    "span",
    "strike",
    "strong",
    "style",
    "sub",
    "summary",
    "sup",
    "svg",
    "table",
    "tbody",
    "td",
    "template",
    "textarea",
    "tfoot",
    "th",
    "thead",
    "time",
    "title",
    "tr",
    "track",
    "tt",
    "u",
    "ul",
    "var",
    "video",
    "wbr",
    "xmp",
};

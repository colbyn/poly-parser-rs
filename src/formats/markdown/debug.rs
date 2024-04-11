use itertools::Itertools;
use tree_formatter::{PrettyTree, ToPrettyTree};

use super::*;

impl ToPrettyTree for Markdown {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        match self {
            Self::Block(x) => x.to_pretty_tree(),
            Self::Inline(x) => x.to_pretty_tree(),
        }
    }
}
impl ToPrettyTree for Inline {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        match self {
            Self::PlainText(x) => x.to_pretty_tree(),
            Self::Link(x) => x.to_pretty_tree(),
            Self::Image(x) => x.to_pretty_tree(),
            Self::Emphasis(x) => x.to_pretty_tree(),
            Self::Highlight(x) => x.to_pretty_tree(),
            Self::Strikethrough(x) => x.to_pretty_tree(),
            Self::Subscript(x) => x.to_pretty_tree(),
            Self::Superscript(x) => x.to_pretty_tree(),
            Self::InlineCode(x) => x.to_pretty_tree(),
            Self::Latex(x) => x.to_pretty_tree(),
            Self::Raw(x) => PrettyTree::key_value("Inline::Raw", x.to_pretty_tree())
        }
    }
}
impl ToPrettyTree for InlineSequence {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        let children = self.0.iter().map(|x| x.to_pretty_tree()).collect_vec();
        tree_formatter::PrettyTree::branch_of("InlineSequence", children)
    }
}
impl ToPrettyTree for Block {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        match self {
            Self::Heading(x) => x.to_pretty_tree(),
            Self::Paragraph(x) => x.to_pretty_tree(),
            Self::Blockquote(x) => x.to_pretty_tree(),
            Self::List(x) => x.to_pretty_tree(),
            Self::FencedCodeBlock(x) => x.to_pretty_tree(),
            Self::HorizontalRule(x) => x.to_pretty_tree(),
            Self::Table(x) => x.to_pretty_tree(),
            Self::Newline(x) => x.to_pretty_tree(),
        }
    }
}
impl ToPrettyTree for inline::PlainText {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("PlainText", &[
            tree_formatter::PrettyTree::key_value("value", self.value.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Link {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Link", &[
            tree_formatter::PrettyTree::key_value("text", self.text.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("url", self.url.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Url {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Url", &[
            tree_formatter::PrettyTree::key_value("destination", self.destination.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("title", self.title.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Image {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Image", &[
            tree_formatter::PrettyTree::key_value("bang", self.bang.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("link", self.link.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Emphasis {
    fn to_pretty_tree(&self) -> PrettyTree {
        tree_formatter::PrettyTree::branch_of("Emphasis", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Highlight {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Highlight", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Strikethrough {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Strikethrough", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Subscript {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Subscript", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Superscript {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Superscript", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::InlineCode {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("InlineCode", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for inline::Latex {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Latex", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::Heading {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Heading", &[
            tree_formatter::PrettyTree::key_value("hash_tokens", self.hash_tokens.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::Paragraph {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Paragraph", &[
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::Blockquote {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Blockquote", &[
            tree_formatter::PrettyTree::key_value("start_delimiters", self.start_delimiters.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::FencedCodeBlock {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("FencedCodeBlock", &[
            tree_formatter::PrettyTree::key_value("fence_start", self.fence_start.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("info_string", self.info_string.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("fence_end", self.fence_end.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::HorizontalRule {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("HorizontalRule", &[
            tree_formatter::PrettyTree::key_value("tokens", self.tokens.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::Table {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("Table", &[
            tree_formatter::PrettyTree::key_value("header", self.header.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("data", self.data.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::table::Header {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("table::Header", &[
            tree_formatter::PrettyTree::key_value("header", self.header.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("separator", self.separator.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::table::SeperatorRow {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("table::SeperatorRow", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("columns", self.columns.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::table::SeperatorRowCell {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("table::SeperatorRowCell", &[
            tree_formatter::PrettyTree::key_value("start_colon", self.start_colon.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("dashes", self.dashes.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_colon", self.end_colon.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("end_delimiter", self.end_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::table::Row {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("table::Row", &[
            tree_formatter::PrettyTree::key_value("start_delimiter", self.start_delimiter.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("cells", self.cells.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::table::RowCell {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("table::RowCell", &[
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("pipe_delimiter", self.pipe_delimiter.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::List {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        match self {
            Self::Unordered { items } => PrettyTree::branch_of(
                "List::Unordered",
                items.iter().map(|x| x.to_pretty_tree()).collect_vec()
            ),
            Self::Ordered { items } => PrettyTree::branch_of(
                "List::Ordered",
                items.iter().map(|x| x.to_pretty_tree()).collect_vec()
            ),
            Self::Task { items } => PrettyTree::branch_of(
                "List::Task",
                items.iter().map(|x| x.to_pretty_tree()).collect_vec()
            ),
        }
    }
}
impl ToPrettyTree for block::list::UnorderedItem {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("list::UnorderedItem", &[
            tree_formatter::PrettyTree::key_value("bullet", self.bullet.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::list::OrderedItem {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("list::OrderedItem", &[
            tree_formatter::PrettyTree::key_value("number", self.number.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("dot", self.dot.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
        ])
    }
}
impl ToPrettyTree for block::list::TaskItem {
    fn to_pretty_tree(&self) -> tree_formatter::PrettyTree {
        tree_formatter::PrettyTree::branch_of("list::TaskItem", &[
            tree_formatter::PrettyTree::key_value("bullet", self.bullet.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("header", self.header.to_pretty_tree()),
            tree_formatter::PrettyTree::key_value("content", self.content.to_pretty_tree()),
        ])
    }
}
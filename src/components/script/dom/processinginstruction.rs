/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::ProcessingInstructionBinding;
use dom::bindings::utils::DOMString;
use dom::characterdata::CharacterData;
use dom::document::AbstractDocument;
use dom::node::{AbstractNode, Node, ProcessingInstructionNodeTypeId};

/// An HTML processing instruction node.
pub struct ProcessingInstruction {
    // FIXME: s/element/characterdata/ https://github.com/mozilla/servo/issues/1594
    element: CharacterData,
    target: DOMString,
}

impl ProcessingInstruction {
    pub fn new_inherited(target: DOMString, data: DOMString, document: AbstractDocument) -> ProcessingInstruction {
        ProcessingInstruction {
            element: CharacterData::new_inherited(ProcessingInstructionNodeTypeId, data, document),
            target: target
        }
    }

    pub fn new(target: DOMString, data: DOMString, document: AbstractDocument) -> AbstractNode {
        let node = ProcessingInstruction::new_inherited(target, data, document);
        Node::reflect_node(@mut node, document, ProcessingInstructionBinding::Wrap)
    }
}

impl ProcessingInstruction {
    pub fn Target(&self) -> DOMString {
        self.target.clone()
    }
}

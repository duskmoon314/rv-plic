var searchIndex = JSON.parse('{\
"rv_plic":{"doc":"RISC-V Platform Level Interrupt Controller","t":[0,0,3,3,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,3,3,3,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,12,12,12,12,11,11,11,11,11,11,11,11,11],"n":["plic","registers","PLIC","Priority","any","borrow","borrow","borrow_mut","borrow_mut","claim","clear_enable","clone","cmp","complete","context_address","disable","enable","eq","fmt","from","from","from","get_context_reserved","get_enable","get_priority","get_threshold","hash","highest","into","into","is_enabled","is_pending","lowest","ne","never","partial_cmp","set_context_reserved","set_enable","set_priority","set_threshold","toggle","try_from","try_from","try_into","try_into","type_id","type_id","Context","Enables","Registers","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","claim_complete","context","enable","enables","from","from","from","into","into","into","pending","priority","reserved","threshold","try_from","try_from","try_from","try_into","try_into","try_into","type_id","type_id","type_id"],"q":["rv_plic","","rv_plic::plic","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","rv_plic::registers","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"d":["Platform Level Interrupt Controller","Platform Level Interrupt Controller Registers","Platform Level Interrupt Controller","Priority of interrupt","(Use in threshold) Any priorities will trigger","","","","","Claim interrupt","Clear a 32bit register in enable of context <code>C</code>","","","Complete interrupt","Get context address","Disable interrupt <code>i</code> for context <code>c</code>","Enable interrupt <code>i</code> for context <code>c</code>","","","","","","Read context reserved","Get a 32bit register in enable of context <code>c</code>","Get interrupt <code>i</code> priority","Get threshold for context <code>c</code>","","The highest priority to trigger an interrupt","","","Check if interrupt <code>i</code> is enabled on context <code>c</code>","Check if interrupt <code>i</code> is pending","The lowest priority to trigger an interrupt","","Never trigger an interrupt","","Write context reserved","Set a 32bit register in enable of context <code>c</code>","Set <code>i</code> priority","Set threshold for context <code>c</code>","Enable/Disable interrupt <code>i</code> for context <code>c</code>","","","","","","","Context","Enable Bitmap","Register Block","","","","","","","0x4 claim/complete for context","base + 0x200000: Priority threshold for context 0base + …","base + 0x002000: Enable bits for sources 0-31 on context 0…","0x00 Interrupt Source #0 to #31 Enable Bits for context …","","","","","","","base + 0x001000: Interrupt Pending bit 0-31base + …","base + 0x000000: Reserved (interrupt source 0 does not …","","0x0 prority threshold for context","","","","","","","","",""],"i":[0,0,0,0,1,2,1,2,1,2,2,1,1,2,2,2,2,1,1,2,1,1,2,2,2,2,1,1,2,1,2,2,1,1,1,1,2,2,2,2,2,2,1,2,1,2,1,0,0,0,3,4,5,3,4,5,5,3,3,4,3,4,5,3,4,5,3,3,5,5,3,4,5,3,4,5,3,4,5],"f":[null,null,null,null,[[],["priority",3]],[[]],[[]],[[]],[[]],[[["usize",15]],[["option",4],["u16",15]]],[[["usize",15]]],[[],["priority",3]],[[["priority",3]],["ordering",4]],[[["usize",15],["u16",15]]],[[["usize",15]],["usize",15]],[[["usize",15],["u16",15]]],[[["usize",15],["u16",15]]],[[["priority",3]],["bool",15]],[[["formatter",3]],["result",6]],[[]],[[["u32",15]]],[[]],[[["usize",15]],["u32",15]],[[["usize",15]],["u32",15]],[[["u16",15]],["priority",3]],[[["usize",15]],["priority",3]],[[]],[[],["priority",3]],[[]],[[]],[[["usize",15],["u16",15]],["bool",15]],[[["u16",15]],["bool",15]],[[],["priority",3]],[[["priority",3]],["bool",15]],[[],["priority",3]],[[["priority",3]],[["option",4],["ordering",4]]],[[["usize",15],["u32",15]]],[[["usize",15],["u32",15]]],[[["priority",3],["u16",15]]],[[["usize",15],["priority",3]]],[[["usize",15],["u16",15]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],null,null,null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]]],"p":[[3,"Priority"],[3,"PLIC"],[3,"Registers"],[3,"Enables"],[3,"Context"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};
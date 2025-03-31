use std::collections::HashSet;
use rustc_hash::FxHashMap as HashMap;
use fancy_regex::Regex;

use crate::CoreBPE;

fn main() {
    // Define a minimal encoder and special tokens
    let encoder: HashMap<Vec<u8>, u32> = HashMap::from([
        (b"hello".to_vec(), 0),
        (b"world".to_vec(), 1),
        (b" ".to_vec(), 2),
    ]);
    let special_tokens_encoder: HashMap<String, u32> = HashMap::from([
        ("<|endoftext|>".to_string(), 50256),
    ]);
    let pattern = r"'s|'t|'re|'ve|'m|'ll|'d| ?[\p{L}]+| ?[\p{N}]+| ?[^\s\p{L}\p{N}]+|\s+(?!\S)|\s+";

    // Initialize CoreBPE
    let core_bpe = CoreBPE::new(encoder, special_tokens_encoder, pattern).unwrap();

    // Define a multi-line test string
    let test_string = "English Speech: 󰀦󰡂󰱒󰱒󰂋󰂋󰂋󰤗󰤗󰁈󰤗󰎒󰎒󰖱󰈌󰈌󰁋󰲛󱂹󰱿󰏺󰕭󰁽󱂒󰝆󰝆󰇩󰍀󰐨󱌕󱌕󰚗󰝡󰔋󰐕󰈱󰯉󰺡󰺡󰺡󰯺󰨽󰨽󰨽󰿊󰿊󰿊󰿊󰿊󰿊󰿊󰝑󰝑󱃊󰋶󰋶󰃅󰽻󰉄󰉄󰫧󰾈󱂗󰚝󰺺󰏽󰭏󰭏󰙎󰳌󰳌󰈢󰆰󰐤󰿗󰋣󱅋󰢟󰢟󰡭󰗬󱄟󰼧󱀜󰍾󰠲󰜿󰰬󰰬󱍂󰮆󰔤󰡀󰎢󰎢󰭑󰎣󰉌󰪻󰉗󰙮󰉗󰷅󰷅󰷅󰷅󰷅󰈲󰮞󰠗󰠗󰠗󰖈󰊡󰊡󱃟󰢖󰀳󰜅󰩜󱋲󰡤󰁞󰰅󰫧󱍩󰗅󰏚󰪤󰶹󰡓󰪂󰜗󰥠󰙏󰻉󱈶󱈶󰪎󰪎󱁋󰇰󰬦󰻑󰧑󰢫󱉩󰦎󱆮󰮵󰹄󰟤󰟤󰾫󰂔󰏣󰽳󰽳󰽳󰵸󰵸󰃑󰔂󰄍󰂗󰆻󰟷󰕱󰋚󱂴󰆓󰙮󰙮󰉗󰷅󰷅󰪻󰷅󰷅󰷅󰮞󰠗󰠗󰪻󰖈󰊡󰊡󰅋󰱛󰅻󰌌󰝁󰒁󰁞󰁞󰫧󰻊󰇠󰚝󰚝󰺺󰔓󰜮󰃦󰃦󰙜󰸗󰸗󱉧󰇉󰓕󰬜󰏃󰈸󱂈󰟙󰘷󰳿󰿗󰕝󰕅󰕅󰕅󰕅󰕅󰞹󰞹󰔐󰢗󰶎󰭵󱀤󰎵󰉤󰉤󰈻󰝆󰣜󰷂󰛨󰛨󰛨󰬫󰾈󱅑󰗅󰜴󱀭󰶹󰪂󰜗󰥠󰀫󰊟󰟻󰂴󰂴󰂴󱍇󱊴󰲟󰊹󰊹󰟕󰟕󰁖󰔈󰵐󰧮󰖉󰿭󰽊󰁏󰮹󰆫󰆫󰫐󰑥󰽬󰋯󰋯󰋯󰦐󰕬󰕬󱊽󰓁󰃕󰜥󰭵󰀩󰣠󰊴󰊴󱋣󱍗󰢗󰀅󰠙󰂯󰒥󰒥󰫀󰻊󰿭󰗅󰭺󰜴󰍀󰁤󰁤󱌕󰚗󰔋󰔋󰽃󰺡󰺡󰺡󰯺󰟾󰨽󱁊󱁊󱁊󱁊󰿊󰃃󰆇󰆇󱅢󰎣󰎣󰉌󰉌󰪻󰪋󰪋󰪋󰪋󰏸󰏸󱋗󱋗󰈾󰈾󰈾󰈾󰈾󰈾󰳩󰈾󱊼󰘤󰘤󰘤󰘤󰵗󰵗󰡪󰃖󰈏󰭭󰥶󰊡󰅋󰸞󰐨󰁤󱌕󰝡󰔋󰂻󰽃󰺡󰺡󱂽󰟾󰟾󰯃󰏇󰏇󰿊󰒣󰒙󰚞󰉴󰈰󰏨󰶡󰳮󰉙󰄂󰢕󰢕󰄨󰭚󰄫󰄫󰺖󰺖󰐂󰀺󰳌󰳌󰭾󰭾󰊖󰳛󰰞󰡓󰙑󰿗󰢷󰔃󰗃󰗃󰗃󰗃󰗃󰍡󱋛󰌷󱈨󱈨󱊲󰡚󰡯󰗴󰐢󰞢󰃛󰶢󰦱󰓇󰯐󰺍󰐀󰈻󰈻󰝆󱀭󰩈󰴨󰑕󱌌󰏲󰹼󱄞󰖕󰖕󱃲󱃲󰙊󰙊󰙊󰟾󰨑󰨑󰨑󰶟󰿶󰿶󰿶󰍇󰏭󰇑󰇑󰇑󰕼󰿇󰠅󰠅󰠅󰎄󰎄󱃁󱃁󰣴󰿒󰏳󰾚󰭺󰏚󰜴󰎵󰉤󰮕󰮕󰳌󰗐󰗐󰗐󰵵󰸢󰸢󰶟󰄫󰺖󰐂󰱑󰀺󰥖󰃀󰃀󱍘󰡏󰡏󱀣󰝆󰅽󰙷󰢵󰅀󰅀󰌤󰌤󰈻󰒰󰭵󰎰󰬽󰉱󰉱󰡁󰫷󰔽󰭇󰐆󰄂󰌎󰌎󰋸󰅾󰀉󰐼󰳇󰿑󰙩󰙫󰭀󰭀󰇼󰒠󱍠󱍠󰶺󰇵󰇙󰥋󰰎󰺻󰌾󱌟󰨎󱅲󰱊󰱊󰱊󰽛󰀇󰉒󰉒󰉒󰣴󰿒󰻾󰥜󰥜󱈕󱃩󱃩󰎊󰠄󰲴󰲴󰀨󰰞󰂱󰪂󰜗󰥠󰀫󰟻󰂴󰂴󰂴󱍇󱍇󱊴󰲟󰊹󰨺󰟕󰟕󰮆󰛶󰡀󰎢󰎢󰎣󰈫󱇴󱇴󰗘󰗘󰪻󰃿󰾦󰾦󰰟󰰟󰰟󰰟󰢡󰢡󰮠󰮠󰆿
English Text: but the owl is not a burglar he is the friend of man there is no other bird that does the farmer so much good as the owl the owl comes out in the dark to get the small animals that are out at that time stealing things from the farmer";

    // Encode the test string
    let tokens = core_bpe.encode_native(test_string, &HashSet::new()).0;

    // Print the encoded tokens
    println!("Encoded tokens for '{}': {:?}", test_string, tokens);
}
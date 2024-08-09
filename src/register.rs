//use strum_macros::EnumIter;
use variant_count::VariantCount;


#[derive(Debug,VariantCount)]
enum Register {
    EAX, 
    ECX,
    EDX,
    EBX,
    ESP,
    EBP,
    ESI,
    EDI,
}
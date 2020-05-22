#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Product {
    pub id: u64,
    pub title: String,
    pub handle: String,
    pub body_html: Option<String>,
    pub vendor: String,
    pub product_type: String,
    pub tags: Vec<String>,
    pub variants: Vec<ProductVariant>,
    pub images: Vec<ProductImage>,
    pub options: Vec<ProductOption>,
    pub published_at: String, // TODO
    pub created_at: String,   // TODO
    pub updated_at: String,   // TODO
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProductVariant {
    pub id: u64,
    pub title: String,
    pub option1: Option<String>,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub sku: Option<String>,
    pub requires_shipping: bool,
    pub taxable: bool,
    pub featured_image: Option<ProductImage>,
    pub available: bool,
    pub price: String,
    pub grams: u32,
    pub compare_at_price: Option<String>,
    pub position: u8,
    pub product_id: u64,
    pub created_at: String, // TODO
    pub updated_at: String, // TODO
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProductImage {
    pub id: u64,
    pub product_id: u64,
    pub position: u8,
    pub alt: Option<String>,
    pub width: u32,
    pub height: u32,
    pub src: String,
    pub variant_ids: Vec<u64>,
    pub created_at: String, // TODO
    pub updated_at: String, // TODO
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ProductOption {
    name: String,
    values: Vec<String>,
    position: u8,
}

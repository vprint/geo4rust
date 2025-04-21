use std::cmp::Reverse;
use gdal::vector::{ Feature, Layer, LayerAccess };
use md5::Digest;

type FieldsName<'a> = Vec<&'a str>;
type FieldsIndex = Vec<i32>;

pub struct AttributeManager<'a> {
    layer: &'a Layer<'a>,
}

impl<'a> AttributeManager<'a> {
    pub fn new(layer: &'a Layer<'a>) -> Self {
        AttributeManager { layer }
    }

    /// Convert a vector of fields name to a vector of fields index.
    /// This method is usefull as GDAL works with index rather than name.
    ///
    /// # Arguments
    ///
    /// * `fields_to_remove` A list of fields name to remove.
    ///
    /// # Returns
    ///
    /// * `FieldsIndex` A list of fields index.
    fn get_fields_index(&self, fields_to_remove: FieldsName) -> FieldsIndex {
        let field_def = self.layer.defn();
        let mut index: FieldsIndex = Vec::new();

        for field_name in fields_to_remove {
            let field_index_result = field_def.field_index(field_name);

            if let Ok(field_index) = field_index_result {
                index.push(field_index as i32);
            }
        }

        index
    }

    /// Delete fields for a given list of fieldnames.
    ///
    /// # Arguments
    ///
    /// * `fields_to_remove` A list of fields name to remove.
    pub fn delete_fields(&mut self, fields_to_remove: FieldsName) {
        let mut fields_index = self.get_fields_index(fields_to_remove);
        fields_index.sort_by_key(|&x| Reverse(x));

        for field_index in fields_index {
            unsafe {
                gdal_sys::OGR_FD_DeleteFieldDefn(self.layer.defn().c_defn(), field_index);
            }
        }
    }

    pub fn has_same_value(a: &Feature, b: &Feature) -> bool {
        AttributeManager::get_attribute_hash(&a) == AttributeManager::get_attribute_hash(&b)
    }

    /// Get an hash of the features attributes values.
    /// This method is usefull to compare feature.
    ///
    /// # Arguments
    ///
    /// * `feature` The feature.
    ///
    /// # Returns
    ///
    /// * `Digest` A raw hash of the feature attributes.
    pub fn get_attribute_hash(feature: &Feature) -> Digest {
        let field_count = feature.field_count();
        let mut hasher = md5::Context::new();

        for i in 0..field_count {
            if let Ok(field_value_option) = feature.field(i) {
                match field_value_option {
                    Some(value) => {
                        let value_str = format!("{:?}", value);
                        hasher.consume(value_str.as_bytes());
                    }
                    None => {
                        hasher.consume(b"None");
                    }
                }
                hasher.consume(b"|");
            }
        }

        md5::Digest::from(hasher.compute())
    }
}

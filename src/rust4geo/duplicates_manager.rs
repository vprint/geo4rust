use std::collections::HashMap;
use gdal::vector::{ Geometry, Layer, LayerAccess };
use crate::rust4geo::attribute_manager::AttributeManager;
use crate::rust4geo::geometry_utils::{ get_extent_as_geometry, is_adjacent };

/// A list of feature Id.
type FeatureIds = Vec<u64>;
type DuplicatesMap = HashMap<u64, FeatureIds>;

/// Manages the detection and handling of duplicate features within a layer.
pub struct DuplicatesManager<'a> {
    layer: &'a mut Layer<'a>,
}

impl<'a> DuplicatesManager<'a> {
    pub fn new(layer: &'a mut Layer<'a>) -> Self {
        DuplicatesManager { layer }
    }

    /// Find all the duplicated neighboors of a layer.
    ///
    /// # Returns
    ///
    /// * `DuplicatesMap` An hashmap of the first feature and the associated duplicates neighboors.
    pub fn find_duplicates_neighboors(&mut self) -> DuplicatesMap {
        let features_ids = self.extract_features_ids();
        let mut duplicates_map: DuplicatesMap = HashMap::new();
        let mut children_hashmap: HashMap<u64, u64> = HashMap::new();

        'a_loop: for a_fid in features_ids {
            println!("Traitement de la feature {a_fid}");
            if children_hashmap.contains_key(&a_fid) {
                continue;
            }

            let bbox_fids = &self.get_features_in_neighboors(a_fid);

            // Process childs
            let childs = Self::get_present(bbox_fids, &duplicates_map);
            for b_fid in childs {
                if self.should_be_added(a_fid, b_fid) {
                    let parent_id = *children_hashmap.get(&b_fid).unwrap();
                    duplicates_map.get_mut(&parent_id).unwrap().push(a_fid);
                    children_hashmap.insert(a_fid, parent_id);
                    continue 'a_loop;
                }
            }

            // Process orphans
            let orphans = Self::get_absent(bbox_fids, &duplicates_map);
            for b_fid in orphans {
                if self.should_be_added(a_fid, b_fid) {
                    duplicates_map.entry(a_fid).or_insert_with(Vec::new).push(b_fid);
                    children_hashmap.insert(b_fid, a_fid);
                }
            }
        }

        duplicates_map
    }

    /// Get all the feature id of a layer
    ///
    /// # Returns
    ///
    /// * `FeatureIds` A list of feature id.
    ///
    fn extract_features_ids(&mut self) -> FeatureIds {
        let mut features_ids: FeatureIds = Vec::new();

        for feature in self.layer.features() {
            if let Some(id) = feature.fid() {
                features_ids.push(id);
            }
        }

        features_ids
    }

    /// Get ids of the features in the bbox.
    ///
    /// # Arguments
    ///
    /// * `a` - The feature.
    ///
    ///  # Returns
    ///
    /// * `FeatureIds` A vector of feature ids.
    fn get_features_in_neighboors(&mut self, id: u64) -> FeatureIds {
        let mut bbox_option: Option<Geometry> = None;

        if let Some(feature) = &self.layer.feature(id) {
            bbox_option = get_extent_as_geometry(&feature);
        }

        match bbox_option {
            Some(bbox) => {
                self.layer.set_spatial_filter(&bbox);
                self.layer.reset_feature_reading();

                let mut result = Vec::with_capacity(self.layer.feature_count() as usize);
                result.extend(
                    self.layer
                        .features()
                        .filter_map(|feature| feature.fid().filter(|&fid| fid != id))
                );

                self.layer.reset_feature_reading();
                result
            }
            None => Vec::new(),
        }
    }

    /// Get the element of a vector that are present in a given hashmap.
    ///
    /// # Arguments
    ///
    /// * `vector_a` - Source vector from which to extract elements.
    /// * `vector_b` - Comparison hashmap.
    ///
    /// # Returns
    ///
    /// A vector of references to elements from `vector_a` that are present in `hashmap` keys.
    pub fn get_present(vector_a: &Vec<u64>, hashmap: &DuplicatesMap) -> Vec<u64> {
        vector_a
            .iter()
            .filter(|item| hashmap.contains_key(item))
            .copied()
            .collect()
    }

    /// Get the element of a vector that are absent from the given hashmap.
    ///
    /// # Arguments
    ///
    /// * `vector_a` - Source vector from which to extract elements.
    /// * `vector_b` - Comparison hashmap.
    ///
    /// # Returns
    ///
    /// A vector of references to elements from `vector_a` that are absent from `hashmap` keys.
    pub fn get_absent(vector_a: &Vec<u64>, hashmap: &DuplicatesMap) -> Vec<u64> {
        vector_a
            .iter()
            .filter(|item| !hashmap.contains_key(item))
            .copied()
            .collect()
    }

    /// Should the feature be added in the list ?
    ///
    /// # Arguments
    ///
    /// * `a_fid` The first feature
    /// * `b_fid` The second feature
    ///
    /// # Returns
    ///
    /// `bool` True if the feature should be added to the duplicate list, false otherwise
    fn should_be_added(&self, a_fid: u64, b_fid: u64) -> bool {
        if a_fid == b_fid {
            return false;
        }

        let a_feat = &self.layer.feature(a_fid).expect("Error while reading feature.");
        let b_feat = &self.layer.feature(b_fid).expect("Error while reading feature.");

        if !AttributeManager::has_same_value(&a_feat, &b_feat) {
            return false;
        }

        if is_adjacent(&a_feat, &b_feat) {
            return true;
        }

        false
    }
}

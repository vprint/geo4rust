mod rust4geo;

use gdal::{ vector::Layer, Dataset };
use rust4geo::{ attribute_manager::AttributeManager, duplicates_manager::DuplicatesManager };

fn main() {
    // Lecture du fichier.
    let ds: Dataset = Dataset::open_ex("src/data/carhab.gpkg", gdal::DatasetOptions {
        open_flags: gdal::GdalOpenFlags::GDAL_OF_UPDATE,
        ..Default::default()
    }).expect("Reading error");

    // Ouverture de la couche.
    let mut layer = ds.layer(0).expect("Layer error");
    remove_unused_attributes(&mut layer);
    remove_duplicates(&mut layer)
}

fn remove_unused_attributes(layer: &mut Layer) {
    // Liste des attributs à supprimer
    let attributes = vec![
        "physio_majoritaire",
        "cd_sig_departement",
        "pourc_physio_maj",
        "filtre_physio",
        "libelle_court",
        "physio_source",
        "id_evenement",
        "id_polygone",
        "surface",
        "id_jdd",
        "cd_hab"
    ];

    let mut attribute_manager: AttributeManager<'_> = AttributeManager::new(layer);
    attribute_manager.delete_fields(attributes);
}

fn remove_duplicates<'a>(layer: &'a mut Layer<'a>) {
    let mut geometry_worker = DuplicatesManager::new(layer);
    let duplicate_list = time_it! {
        geometry_worker.find_duplicates_neighboors()
    };
    // todo :
}

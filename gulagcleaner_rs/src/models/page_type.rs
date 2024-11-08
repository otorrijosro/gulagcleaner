use std::{collections::HashSet, error::Error};

use lopdf::{Document, ObjectId};

use super::method::{get_images, get_xobjs};

#[derive(Default)]
#[derive(Debug)]
/// Represents the different methods used in the Gulag Cleaner application.
pub enum PageType {
    BannerAds,
    FullPageAds,
    Watermark,
    #[default]
    Idk,
}

pub const LOGO_DIMS: [(i64, i64); 9] = [(71, 390), (37, 203), (73, 390),(23,130),(24,130),(19,109),(20,109),(72,391),(24,129)];

const HORIZONTAL_BANNER_DIMS: [(i64, i64); 10] = [
    (247, 1414),
    (213, 1219),
    (215, 1219),
    (249, 1414),
    (217, 1240),
    (147, 1757),
    (221, 1240),
    (136, 780),
    (137,780),
    (218,1241)
];
const VERTICAL_BANNER_DIMS: [(i64, i64); 11] = [
    (1753, 170),
    (1518, 248),
    (1520, 147),
    (1753, 177),
    (1751, 171),
    (1537, 147),
    (1093, 217),
    (1534, 150),
    (970, 92),
    (969,93),
    (1538, 148)
];
const FULL_PAGE_DIMS: [(i64, i64); 10] = [
    (842, 595),
    (1754, 1240),
    (2526, 1785),
    (1733, 1219),
    (3508, 2480),
    (2339, 1653),
    (1785, 2526),
    (1109, 782),
    (1109,784),
    (1759,1241)
];

impl PageType {
    /// Get the type of a page based on its content.
    ///
    /// This function takes a document and a page ID as input and returns the type of the page.
    /// The page type is determined by analyzing the images present in the page.
    /// It checks for the presence of specific image dimensions to identify different types of pages,
    /// such as banner ads, full-page ads, watermarks, or unknown types.
    ///
    /// # Arguments
    ///
    /// * `doc` - A reference to the document containing the page.
    /// * `page` - A reference to the ID of the page.
    ///
    /// # Returns
    ///
    /// A `Result` containing the `PageType` of the page if successful, or a `Box<dyn Error>` if an error occurs.
    pub fn get_page_type(doc: &Document, page: &ObjectId) -> Result<PageType, Box<dyn Error>> {
        let xobjs = get_xobjs(doc, page)?;
        let images = get_images(doc, xobjs)?;
        println!("{:?}", images);
        // let has_logo = !LOGO_DIMS
        //     .iter()
        //     .collect::<HashSet<_>>()
        //     .intersection(&images.iter().collect::<HashSet<_>>())
        //     .collect::<Vec<_>>()
        //     .is_empty();

        let has_horizontal_banner = !HORIZONTAL_BANNER_DIMS
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&images.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .is_empty();

        let has_vertical_banner = !VERTICAL_BANNER_DIMS
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&images.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .is_empty();

        let has_full_page = !FULL_PAGE_DIMS
            .iter()
            .collect::<HashSet<_>>()
            .intersection(&images.iter().collect::<HashSet<_>>())
            .collect::<Vec<_>>()
            .is_empty();

        if has_horizontal_banner && has_vertical_banner {
            Ok(PageType::BannerAds)
        } else if has_full_page {
            Ok(PageType::FullPageAds)
        } else {
            let annots = doc.get_page_annotations(*page)?;
            
            let wuolah_annot =  annots.iter().filter(|x| is_annots_wuolah(x,doc)).filter( |x| x.get(b"Rect").unwrap().as_array().unwrap()[0] == lopdf::Object::Integer(0) || x.get(b"Rect").unwrap().as_array().unwrap()[0] == lopdf::Object::Real(0.0));
            // For each wuolah annot, check if substrings are present in any of the URI
            println!("{:?}", wuolah_annot);
            let mut bannercounter = 0;
            let mut hasfooter = false;
            for annot in wuolah_annot {
                if let Ok(action) = annot.get(b"A") {
                    if let Ok(action_dict) = doc.dereference(action) {
                        if let Ok(uri) = action_dict.1.as_dict().unwrap().get(b"URI") {
                            if let Ok(url) = doc.dereference(uri) {
                                let url_str = url.1.as_string().unwrap();
                                if url_str.contains("adU=2") {
                                    bannercounter += 1;
                                }
                                if url_str.contains("adU=3") {
                                    hasfooter = true;
                                }
                            }
                        }
                    }
                }
            }
            println!("{}", bannercounter);
            if bannercounter == 1 {
                return Ok(PageType::Watermark)
            }
            if bannercounter > 1 {
                return Ok(PageType::BannerAds)
            } 
            if hasfooter {
                return Ok(PageType::Watermark)
            }
            Ok(PageType::Idk)
        }
    }
}

fn is_annots_wuolah(annot: &&&lopdf::Dictionary, doc: &lopdf::Document) -> bool {
    match annot.get(b"A") {
        Ok(x) => {
            match doc.dereference(x).unwrap().1.as_dict().unwrap().get(b"URI") {
                Ok(y) => {
                    let url = doc.dereference(y).unwrap().1.as_string().unwrap();
                    if url.contains("track.wlh.es"){
                        !(url.contains("apuntes"))
                    } else {
                        false
                    }
                },
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
}
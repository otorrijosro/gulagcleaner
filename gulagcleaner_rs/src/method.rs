use std::collections::HashSet;

use lopdf::Document;

pub enum Method {
    New(Vec<Vec<(u32, u16)>>, Vec<u32>),
    Naive
}

impl Method {
    pub fn new(doc: &Document, force_naive: u8) -> Self {
        //0 for auto, 1 for new, 3 for naive
        if force_naive == 1 {
            return Self::Naive
        }
        let pages = doc.get_pages();
        let content_list: Vec<Vec<(u32, u16)>> = pages
            .iter()
            .map(|x| doc.get_page_contents(*x.1))
            .filter(|x| x.len() > 1)
            .collect();
        let to_delete: Vec<u32> = pages
            .iter()
            .filter(|x| {
                let contents = doc.get_page_contents(*x.1);
                
                if contents.len() == 1 {
                    return true;
                } else {
                    return false;
                }
            })
            .map(|x| *x.0)
            .collect();

        if content_list.len() > 1
            && content_list[0]
                .iter()
                .collect::<HashSet<_>>()
                .intersection(&content_list[1].iter().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
                .len()
                > 1
        {
            return Self::New(content_list, to_delete);
        }
        Self::Naive
    }
}
use reqwest;

#[allow(non_camel_case_types)]
pub struct tldr {
   pub search: String,
}

impl tldr {
    pub async fn get_tldr(&self) -> String {
        let res = reqwest::get(format!(
            "https://raw.githubusercontent.com/tldr-pages/tldr/main/pages/linux/{}.md",
            self.search
        )).await.unwrap().text().await.unwrap();

        res
    }
}

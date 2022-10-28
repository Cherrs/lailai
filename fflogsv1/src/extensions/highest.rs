use crate::{FFError, FF14};

impl FF14 {
    pub async fn get_highest(
        &self,
        character_name: &str,
        server_name: &str,
        server_region: &str,
    ) -> Result<Vec<GetHighestDataDto>, FFError> {
        let mut data = self
            .character_parses(
                character_name,
                server_name,
                server_region,
                "rdps",
                None,
                "historical",
            )
            .await?;
        let mut result = Vec::new();
        data.sort_unstable_by_key(|x| x.encounter_id);
        let mut datax = data.group_by_mut(|x, y| x.encounter_id == y.encounter_id);
        for x in datax.by_ref() {
            x.sort_by(|a, b| a.difficulty.cmp(&b.difficulty));
            let o = x.group_by(|a, b| a.difficulty == b.difficulty);
            for i in o {
                let _o = i
                    .iter()
                    .max_by(|a, b| a.percentile.partial_cmp(&b.percentile).unwrap())
                    .unwrap();
                let _r = GetHighestDataDto {
                    bossname: _o.encounter_name.to_string(),
                    difficulty: _o.difficulty,
                    rank: _o.percentile,
                    rdps: _o.total,
                    spec: _o.spec.to_string(),
                };
                result.push(_r);
            }
        }
        result.sort_unstable_by(|x, y| y.rank.partial_cmp(&x.rank).unwrap());
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::FF14;

    #[tokio::test]
    async fn it_works() {
        let ff14client = FF14::new(&env::var("logskey").unwrap());
        let dtos = ff14client.get_highest("Iker", "琥珀原", "cn").await;
        println!("{dtos:#?}");
    }
}
#[derive(Debug)]
pub struct GetHighestDataDto {
    pub bossname: String,
    pub rank: f32,
    pub rdps: f32,
    pub spec: String,
    pub difficulty: i32,
}

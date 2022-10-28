use crate::FF14;

impl FF14 {
    pub async fn get_fight(
        &self,
        code: &str,
        fight: i64,
    ) -> Result<GetFightDto, Box<dyn std::error::Error>> {
        let report = self.fights_report(code).await?;
        let report = report
            .fights
            .iter()
            .find(|x| x.id == fight)
            .expect("没有找到这场战斗");
        let dead = self
            .tables_report_deaths(code, report.start_time, report.end_time)
            .await?;
        let deaths = dead.entries.iter().map(|x| Deaths {
            name: x.name.to_string(),
            deathname: match &x.killing_blow {
                Some(c) => c.name.to_string(),
                None => "未知？".to_string(),
            },
        });
        Ok(GetFightDto {
            fiexdtime: report.end_time - report.start_time,
            deaths: deaths.collect(),
        })
    }
}
#[derive(Debug)]
pub struct GetFightDto {
    pub fiexdtime: i64,
    pub deaths: Vec<Deaths>,
}
#[derive(Debug)]
pub struct Deaths {
    pub name: String,
    pub deathname: String,
}

#[cfg(test)]
mod tests {

    use crate::FF14;
    #[tokio::test]
    async fn get_fight() {
        let ff14client = FF14::new("ddac920f50d421116883220e4d149fdf");
        let dtos = ff14client.get_fight("1MahAGrFRJ9BVqYK", 1).await.unwrap();
        println!("{dtos:?}");
    }
}

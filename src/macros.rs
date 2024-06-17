#[macro_export] macro_rules! transactional {
    ($($db:ident).*, |$tx:ident| $scope:expr) => {
        async {
            use abstract_db::{Db, DbTransaction};

            let tx = $($db).*.begin().await;
            match tx {
                Ok(mut tx) => {
                    let result = {
                        let mut $tx = &mut tx;
                        $scope.await
                    };
                    match result {
                        Ok(result) => if let Err(err) = tx.commit().await {
                                Err(err)
                            } else {
                                Ok(result)
                            },
                        Err(err) => {
                            tx.rollback().await.ok();
                            Err(err)
                        }
                    }
                },
                Err(err) => Err(err)
            }
        }
    };
}
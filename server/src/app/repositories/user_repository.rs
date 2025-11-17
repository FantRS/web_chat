use sqlx::PgExecutor;

pub async fn get<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn get_by<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn create<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn update<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

pub async fn delete<'c, E>(exec: E)
where
    E: PgExecutor<'c>,
{
}

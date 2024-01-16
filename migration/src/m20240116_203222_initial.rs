use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Team table
        manager
            .create_table(
                Table::create()
                    .table(Team::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Team::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Team::Name).string().not_null())
                    .col(ColumnDef::new(Team::FoundingDate).date().not_null())
                    .col(ColumnDef::new(Team::Address).string().not_null())
                    .col(ColumnDef::new(Team::ZipCode).string().not_null())
                    .to_owned(),
            )
            .await?;

        // Swimmer table
        manager
            .create_table(
                Table::create()
                    .table(Swimmer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Swimmer::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Swimmer::Name).string().not_null())
                    .col(ColumnDef::new(Swimmer::DateOfBirth).date().not_null())
                    .col(ColumnDef::new(Swimmer::Team).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_swimmer_team")
                            .from(Swimmer::Table, Swimmer::Team)
                            .to(Team::Table, Team::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // Competition table
        manager
            .create_table(
                Table::create()
                    .table(Competition::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Competition::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Competition::Name).string().not_null())
                    .col(ColumnDef::new(Competition::Date).date().not_null())
                    .col(ColumnDef::new(Competition::Location).string().not_null())
                    .col(ColumnDef::new(Competition::Type).string().not_null())
                    .col(ColumnDef::new(Competition::Host).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_competition_host")
                            .from(Competition::Table, Competition::Host)
                            .to(Team::Table, Team::Id),
                    )
                    .to_owned(),
            )
            .await?;

        // SwimTime table
        manager
            .create_table(
                Table::create()
                    .table(SwimTime::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(SwimTime::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(SwimTime::Competition).integer().not_null())
                    .col(ColumnDef::new(SwimTime::Distance).integer().not_null())
                    .col(ColumnDef::new(SwimTime::Stroke).string().not_null())
                    .col(ColumnDef::new(SwimTime::Time).integer().not_null())
                    .col(ColumnDef::new(SwimTime::Swimmer).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_swimtime_competition")
                            .from(SwimTime::Table, SwimTime::Competition)
                            .to(Competition::Table, Competition::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_swimtime_swimmer")
                            .from(SwimTime::Table, SwimTime::Swimmer)
                            .to(Swimmer::Table, Swimmer::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop tables in reverse order
        manager
            .drop_table(Table::drop().table(SwimTime::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Competition::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Swimmer::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Team::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Swimmer {
    Table,
    Id,
    Name,
    DateOfBirth,
    Team,
}

#[derive(DeriveIden)]
enum Team {
    Table,
    Id,
    Name,
    FoundingDate,
    Address,
    ZipCode,
    // Add other columns as needed
}

#[derive(DeriveIden)]
enum SwimTime {
    Table,
    Id,
    Competition,
    Time,
    Distance,
    Stroke,
    Swimmer,
}

#[derive(DeriveIden)]
enum Competition {
    Table,
    Id,
    Name,
    Date,
    Location,
    Type,
    Host,
}

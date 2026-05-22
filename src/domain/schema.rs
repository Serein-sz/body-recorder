#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessModel {
    ServiceRole,
    Anon,
}

const BASE_SCHEMA_SQL: &str = r#"create table if not exists public.weight_records (
  id uuid primary key default gen_random_uuid(),
  record_date date not null unique,
  weight_kg numeric(5,2) not null check (weight_kg > 0 and weight_kg < 1000),
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

create or replace function public.set_updated_at()
returns trigger
language plpgsql
as $$
begin
  new.updated_at = now();
  return new;
end;
$$;

drop trigger if exists trg_weight_records_updated_at on public.weight_records;

create trigger trg_weight_records_updated_at
before update on public.weight_records
for each row
execute function public.set_updated_at();

alter table public.weight_records enable row level security;
"#;

const SERVICE_ROLE_ACCESS_SQL: &str = r#"revoke all on table public.weight_records from anon, authenticated;
grant select, insert, update, delete on table public.weight_records to service_role;
"#;

const ANON_ACCESS_SQL: &str = r#"grant select, insert, update, delete on table public.weight_records to anon;

drop policy if exists "anon can select weight records" on public.weight_records;
drop policy if exists "anon can insert weight records" on public.weight_records;
drop policy if exists "anon can update weight records" on public.weight_records;
drop policy if exists "anon can delete weight records" on public.weight_records;

create policy "anon can select weight records"
on public.weight_records
for select
to anon
using (true);

create policy "anon can insert weight records"
on public.weight_records
for insert
to anon
with check (true);

create policy "anon can update weight records"
on public.weight_records
for update
to anon
using (true)
with check (true);

create policy "anon can delete weight records"
on public.weight_records
for delete
to anon
using (true);
"#;

pub fn schema_sql(access: AccessModel) -> String {
    let access_sql = match access {
        AccessModel::ServiceRole => SERVICE_ROLE_ACCESS_SQL,
        AccessModel::Anon => ANON_ACCESS_SQL,
    };

    format!("{BASE_SCHEMA_SQL}\n{access_sql}")
}

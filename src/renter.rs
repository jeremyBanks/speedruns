use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::{BTreeMap, BTreeSet},
    default::Default,
    error::Error,
    fmt::Debug,
    hash::Hash,
};

use serde::Serialize;

type OrderedMap<Key, Value> = std::collections::BTreeMap<Key, Value >;
type OrderedSet<Value> = std::collections::BTreeSet<Value>;
type HashMap<Key, Value> = std::collections::HashMap<Key, Value>;
type HashSet<Value> = std::collections::HashSet<Value>;

mod foo {
    struct User {
        id: u64,
        name: String,
    }

    noloads! {
        database: Database,
        indices: Indices {
            users_by_name: OrderedMap<u64, &'tables User>,
        }
        tables: Tables {
            user: User {
                pk(user) -> Id64 {
                    user.id()
                }
                index(user) {
                    indices.user_by_name.insert(user.id(), &user)
                }
                deindex(user) {
                    indices.user_by_name.remove(user.id())?
                }
            }
        }
    }
}

// A record is a value stored in a [Table], with a unique primary key.
pub trait Record: Sized + Serialize + Debug {
    type PK: Debug + Ord + Sized;
    fn pk(&self) -> Self::PK;

    fn add_indices(
        &self,
        indices: &mut Indices,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn remove_indices(
        &self,
        indices: &mut Indices,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    
    fn replace_indices(
        &self,
        old: &Self,
        indices: &mut Indices,
    ) -> Result<(), Box<dyn Error>> {
        old.remove_indices(&mut indices)?;
        self.add_indices(&mut indices)?;
        Ok(())
    }
}

// Wraps a [Record] to order and equality it by primary key.
pub struct Row<T: Record> {
    record: T,
}

impl<T: Record> Ord for Row<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.record.pk().cmp(&other.record.pk())
    }
}
impl<T: Record> Eq for Row<T> {}
impl<T: Record> PartialOrd for Row<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.record.pk().cmp(&other.record.pk()))
    }
}
impl<T: Record> PartialEq for Row<T> {
    fn eq(&self, other: &Self) -> bool {
        self.record.pk() == other.record.pk()
    }
}

pub struct Tables {
    posts: BTreeSet<Row<Post>>,
}

pub struct Indices<'tables> {
    tables:           &'tables Tables,
    posts_by_user_id: BTreeMap<&'tables str, BTreeSet<&'tables Post>>,
}

pub struct Post {
    id:      u64,
    user_id: u64,
}

impl Record for Post {
    type PK = u64;

    fn pk(&self) -> Self::PK {
        self.id
    }
}

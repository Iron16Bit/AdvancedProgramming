use std::marker::PhantomData;

struct User<const R: Role> {    //R is a value of type Role, which is u8
    //_role: PhantomData<R>;
}

fn f() {
    //Since Role is a u8, we can directly use a u8
    let u1 : User<1> = User{};
    let u1 : User<2> = User{};
    let u1 : User<20> = User{};
}

trait Yes{}

struct Implied<const Left: Role, const Right: Role> {}

impl Yes for Implied<Admin, Admin> {}
impl Yes for Implied<Admin, SuperUser> {}
impl Yes for Implied<Admin, RegUser> {}
impl Yes for Implied<SuperUser, SuperUser> {}
impl Yes for Implied<SuperUser, RegUser> {}
impl Yes for Implied<RegUser, RegUser> {}

impl<const R: Role> User<R> where Implied<R, RegUser>: Yes {
    fn reg_user_thing(&self) {}
}

impl<const R: Role> User<R> where Implied<R, SuperUser>: Yes {
    fn sup_user_thing(&self) {}
}

impl<const R: Role> User<R> where Implied<R, Admin>: Yes {
    fn admin_thing(&self) {}
}

impl<const R: Role> User<R> {
    fn pub_thing(&self) {}
}

// fn admin_thing<const R>(user: &User<R>) where R >= Admin {
//
// }
//
// fn super_user_thing<const R>(user: &User<R>) where R >= SuperUser {
//
// }

const Admin: u8 = 2;
const SuperUser: u8 = 1;
const RegUser: u8 = 0;
type Role = u8;

pub fn main() {
    let user1: User<RegUser> = User{};
    user1.reg_user_thing();

    let user2: User<SuperUser> = User{};
    user2.reg_user_thing();
    user2.sup_user_thing();

    let user3: User<Admin> = User{};
    user3.reg_user_thing();
    user3.sup_user_thing();
    user3.admin_thing();

    let no_user: User<99> = User{};
    no_user.pub_thing();
}
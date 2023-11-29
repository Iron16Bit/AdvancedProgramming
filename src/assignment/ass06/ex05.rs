// use std::marker::PhantomData;
// trait CompileTimeNode{
//     type LeftType;
//     type RightType;
//     fn is_none() -> bool;
// }
// struct NullNode{}
// struct Node<L,R>
//     where L: CompileTimeNode,
//           R: CompileTimeNode
// {
//     left: PhantomData<L>,
//     right: PhantomData<R>
// }
//
// impl CompileTimeNode for NullNode {
//     type LeftType = None;
//     type RightType = None;
//
//     fn is_none() -> bool {
//         return true;
//     }
// }
//
// impl<L,R> CompileTimeNode for Node<L,R> {
//     type LeftType = L;
//     type RightType = R;
//
//     fn is_none() -> bool {
//         return false;
//     }
// }
//
// fn count_nodes<T>() -> usize{
//     todo!()
// }
//
// pub fn test() {
//     let left = NullNode{};
//     let root : Node<i32, i32> = Node{left: PhantomData, right: PhantomData};
// }

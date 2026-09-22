module BNSearchTree2List;
create OUT aList: MMList from IN aBNST : MMBNSearchTree;


rule Tree2List{
	from  t: MMBNSearchTree(isNotEmpty)
		// convert the tree to a list
		// using the in-order traversal
		// left, root, right
		// and add the elements to the list
		to  m: MMList(
			m.add(t.getLeft().toList())
			m.add(t.getRoot())
			m.add(t.getRight().toList())
		)
		
}
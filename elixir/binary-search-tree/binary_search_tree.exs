defmodule BinarySearchTree do
  @type bst_node :: %{data: any, left: bst_node | nil, right: bst_node | nil}

  @doc """
  Create a new Binary Search Tree with root's value as the given 'data'
  """
  @spec new(any) :: bst_node
  def new(data) do
    %{data: data}
  end

  @doc """
  Creates and inserts a node with its value as 'data' into the tree.
  """
  @spec insert(bst_node, any) :: bst_node
  def insert(tree, data) do
    cond do
      tree.data >= data -> insert_at(tree, data, :left)
      tree.data < data -> insert_at(tree, data, :right)
    end
  end

  defp insert_at(tree, data, key) do
    node =
      if Map.has_key?(tree, key) do
        tree |> Map.get(key) |> insert(data)
      else
        new(data)
      end

    tree |> Map.put(key, node)
  end

  @doc """
  Traverses the Binary Search Tree in order and returns a list of each node's data.
  """
  @spec in_order(bst_node) :: [any]
  def in_order(tree) do
    in_order_tree(tree)
  end

  defp in_order_tree(nil) do
    []
  end

  defp in_order_tree(tree) do
    left = Map.get(tree, :left) |> in_order_tree
    right = Map.get(tree, :right) |> in_order_tree
    data = Map.get(tree, :data)
    Enum.concat([left, [data], right])
  end
end

defmodule LinkedList do
  @opaque t :: tuple()
  @sentinel {}

  @doc """
  Construct a new LinkedList
  """
  @spec new() :: t
  def new() do
    @sentinel
  end

  @doc """
  Push an item onto a LinkedList
  """
  @spec push(t, any()) :: t
  def push(list, elem) do
    {elem, list}
  end

  @doc """
  Calculate the length of a LinkedList
  """
  @spec length(t) :: non_neg_integer()
  def length(@sentinel), do: 0
  def length({_, t}) do
    1 + __MODULE__.length(t)
  end

  @doc """
  Determine if a LinkedList is empty
  """
  @spec empty?(t) :: boolean()
  def empty?(@sentinel), do: true
  def empty?(_) do
    false
  end

  @doc """
  Get the value of a head of the LinkedList
  """
  @spec peek(t) :: {:ok, any()} | {:error, :empty_list}
  def peek(@sentinel), do: {:error, :empty_list}
  def peek({h, _}) do
     {:ok, h}
   end

  @doc """
  Get tail of a LinkedList
  """
  @spec tail(t) :: {:ok, t} | {:error, :empty_list}
  def tail(@sentinel), do: {:error, :empty_list}
  def tail({_, t}) do
     {:ok, t}
   end

  @doc """
  Remove the head from a LinkedList
  """
  @spec pop(t) :: {:ok, any(), t} | {:error, :empty_list}
  def pop(@sentinel), do: {:error, :empty_list}
  def pop({h, t}) do
     {:ok, h, t}
   end

  @doc """
  Construct a LinkedList from a stdlib List
  """
  @spec from_list(list()) :: t
  def from_list([]), do: @sentinel
  def from_list(list) do
     {hd(list), from_list(tl(list))}
   end

  @doc """
  Construct a stdlib List LinkedList from a LinkedList
  """
  @spec to_list(t) :: list()
  def to_list(@sentinel), do: []
  def to_list({h, t}) do
    [h | to_list(t)]
  end

  @doc """
  Reverse a LinkedList
  """
  @spec reverse(t) :: t
  def reverse(list) do
    reverse(list, new())
  end
  defp reverse(@sentinel, list), do: list
  defp reverse({h, t}, list) do
    reverse(t, {h, list})
  end
end

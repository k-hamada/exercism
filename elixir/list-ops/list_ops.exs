defmodule ListOps do
  # Please don't use any external modules (especially List) in your
  # implementation. The point of this exercise is to create these basic functions
  # yourself.
  #
  # Note that `++` is a function from an external module (Kernel, which is
  # automatically imported) and so shouldn't be used either.

  @spec count(list) :: non_neg_integer
  def count(l) do
    count_impl(l, 0)
  end

  defp count_impl([], count) do
    count
  end

  defp count_impl([_ | t], c) do
    count_impl(t, c + 1)
  end

  @spec reverse(list) :: list
  def reverse(l) do
    reverse_impl(l, [])
  end

  defp reverse_impl([], acc) do
    acc
  end

  defp reverse_impl([h | t], acc) do
    reverse_impl(t, [h | acc])
  end

  @spec map(list, (any -> any)) :: list
  def map(l, f) do
    map_impl(l, f, [])
  end

  defp map_impl([], _, acc) do
    reverse(acc)
  end

  defp map_impl([h | t], f, acc) do
    map_impl(t, f, [f.(h) | acc])
  end

  @spec filter(list, (any -> as_boolean(term))) :: list
  def filter(l, f) do
    filter_impl(l, f, [])
  end

  defp filter_impl([], _, acc) do
    reverse(acc)
  end

  defp filter_impl([h | t], f, acc) do
    if f.(h) do
      filter_impl(t, f, [h | acc])
    else
      filter_impl(t, f, acc)
    end
  end

  @type acc :: any
  @spec reduce(list, acc, (any, acc -> acc)) :: acc
  def reduce(l, acc, f) do
    reduce_impl(l, acc, f)
  end

  defp reduce_impl([], acc, _) do
    acc
  end

  defp reduce_impl([h | t], acc, f) do
    reduce_impl(t, f.(h, acc), f)
  end

  @spec append(list, list) :: list
  def append(a, b) do
    append_impl(a, b)
  end

  defp append_impl(a, b) do
    reduce_impl(reverse(a), b, &[&1 | &2])
  end

  @spec concat([[any]]) :: [any]
  def concat(ll) do
    concat_impl(ll, [])
  end

  defp concat_impl([], acc) do
    reverse(acc)
  end

  defp concat_impl([h | t], acc) when is_list(h) do
    concat_impl(t, append(reverse(h), acc))
  end

  defp concat_impl([h | t], acc) do
    concat_impl(t, [h | acc])
  end
end

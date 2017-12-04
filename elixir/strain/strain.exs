defmodule Strain do
  @doc """
  Given a `list` of items and a function `fun`, return the list of items where
  `fun` returns true.

  Do not use `Enum.filter`.
  """
  @spec keep(list :: list(any), fun :: ((any) -> boolean)) :: list(any)
  def keep(list, fun) do
    keep_list(list, fun)
  end

  defp keep_list([head | tail], fun) do
    if (fun.(head)) do
      [head | keep_list(tail, fun)]
    else
      keep_list(tail, fun)
    end
  end

  defp keep_list([], _fun) do
    []
  end

  @doc """
  Given a `list` of items and a function `fun`, return the list of items where
  `fun` returns false.

  Do not use `Enum.reject`.
  """
  @spec discard(list :: list(any), fun :: ((any) -> boolean)) :: list(any)
  def discard(list, fun) do
    discard_list(list, fun)
  end

  defp discard_list([head | tail], fun) do
    if (!fun.(head)) do
      [head | discard_list(tail, fun)]
    else
      discard_list(tail, fun)
    end
  end

  defp discard_list([], _fun) do
    []
  end
end

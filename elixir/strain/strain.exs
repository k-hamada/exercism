defmodule Strain do
  @doc """
  Given a `list` of items and a function `fun`, return the list of items where
  `fun` returns true.

  Do not use `Enum.filter`.
  """
  @spec keep(list :: list(any), fun :: ((any) -> boolean)) :: list(any)
  def keep(list, fun) do
    strain_list(list, fun)
  end

  @doc """
  Given a `list` of items and a function `fun`, return the list of items where
  `fun` returns false.

  Do not use `Enum.reject`.
  """
  @spec discard(list :: list(any), fun :: ((any) -> boolean)) :: list(any)
  def discard(list, fun) do
    strain_list(list, &(!fun.(&1)))
  end

  defp strain_list([head | tail], fun) do
    if (fun.(head)) do
      [head | strain_list(tail, fun)]
    else
      strain_list(tail, fun)
    end
  end

  defp strain_list([], _fun) do
    []
  end
end

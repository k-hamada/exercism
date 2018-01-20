defmodule ETL do
  @doc """
  Transform an index into an inverted index.

  ## Examples

  iex> ETL.transform(%{"a" => ["ABILITY", "AARDVARK"], "b" => ["BALLAST", "BEAUTY"]})
  %{"ability" => "a", "aardvark" => "a", "ballast" => "b", "beauty" =>"b"}
  """
  @spec transform(map) :: map
  def transform(input) do
    input
    |> Enum.flat_map(fn {k, v} -> Enum.map(v, &{&1, k}) end)
    |> Map.new(fn {k, v} -> {String.downcase(k), v} end)
  end
end

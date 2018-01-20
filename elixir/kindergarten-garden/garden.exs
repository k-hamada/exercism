defmodule Garden do
  @doc """
    Accepts a string representing the arrangement of cups on a windowsill and a
    list with names of students in the class. The student names list does not
    have to be in alphabetical order.

    It decodes that string into the various gardens for each student and returns
    that information in a map.
  """
  @students ~w(alice bob charlie david eve fred ginny harriet ileana joseph kincaid larry)a
  @table %{"C" => :clover, "G" => :grass, "R" => :radishes, "V" => :violets}
  @width 2

  @spec info(String.t(), list) :: map
  def info(info_string, student_names \\ @students) do
    student_names
    |> to_plant_map(&plant_at(plant_lines(info_string), &1))
  end

  defp to_plant_map(names, plant_getter) do
    names
    |> Enum.sort()
    |> Enum.with_index()
    |> Map.new(&new_plant_map(&1, plant_getter))
  end

  defp new_plant_map({name, index}, plant_getter) do
    {name, plant_getter.(index)}
  end

  defp plant_lines(info) do
    info
    |> String.split("\n")
    |> Enum.map(&String.graphemes/1)
  end

  defp plant_at(plant_lines, index) do
    plant_lines
    |> Enum.flat_map(&Enum.slice(&1, index * @width, @width))
    |> Enum.map(&@table[&1])
    |> List.to_tuple()
  end
end

defmodule RobotSimulator do
  defmodule Robot do
    defstruct [:direction, :position]
  end

  @doc """
  Create a Robot Simulator given an initial direction and position.

  Valid directions are: `:north`, `:east`, `:south`, `:west`
  """
  @spec create(direction :: atom, position :: {integer, integer}) :: any
  def create(direction \\ :north, position \\ {0, 0}) do
    do_create(direction, position)
  end

  defp do_create(direction, {x, y} = position)
       when direction in ~w(north east south west)a and is_integer(x) and is_integer(y) do
    %Robot{direction: direction, position: position}
  end

  defp do_create(direction, _) when direction not in ~w(north east south west)a do
    {:error, "invalid direction"}
  end

  defp do_create(_, _) do
    {:error, "invalid position"}
  end

  @doc """
  Simulate the robot's movement given a string of instructions.

  Valid instructions are: "R" (turn right), "L", (turn left), and "A" (advance)
  """
  @spec simulate(robot :: any, instructions :: String.t()) :: any
  def simulate(robot, instructions) do
    instructions
    |> String.graphemes()
    |> Enum.reduce(robot, &do_simulate/2)
  end

  defp do_simulate("A", %Robot{direction: direction, position: {x, y}}) do
    case direction do
      :north -> create(direction, {x, y + 1})
      :east -> create(direction, {x + 1, y})
      :south -> create(direction, {x, y - 1})
      :west -> create(direction, {x - 1, y})
    end
  end

  defp do_simulate("L", %Robot{direction: direction, position: position}) do
    case direction do
      :north -> create(:west, position)
      :east -> create(:north, position)
      :south -> create(:east, position)
      :west -> create(:south, position)
    end
  end

  defp do_simulate("R", %Robot{direction: direction, position: position}) do
    case direction do
      :north -> create(:east, position)
      :east -> create(:south, position)
      :south -> create(:west, position)
      :west -> create(:north, position)
    end
  end

  defp do_simulate(_, _) do
    {:error, "invalid instruction"}
  end

  @doc """
  Return the robot's direction.

  Valid directions are: `:north`, `:east`, `:south`, `:west`
  """
  @spec direction(robot :: any) :: atom
  def direction(robot) do
    robot.direction
  end

  @doc """
  Return the robot's position.
  """
  @spec position(robot :: any) :: {integer, integer}
  def position(robot) do
    robot.position
  end
end

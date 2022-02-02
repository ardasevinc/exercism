EXPECTED_BAKE_TIME = 40
LAYER_PREPARATION_TIME = 2


def bake_time_remaining(elapsed_bake_time: int) -> int:
    """Calculate the bake time remaining.

    :param elapsed_bake_time: baking time already elapsed.
    :return: remaining bake time derived from 'EXPECTED_BAKE_TIME'.

    Function that takes the actual minutes the lasagna has been in the oven as
    an argument and returns how many minutes the lasagna still needs to bake
    based on the `EXPECTED_BAKE_TIME`.
    """
    return EXPECTED_BAKE_TIME - elapsed_bake_time


def preparation_time_in_minutes(num_of_layers: int) -> int:
    """Calculates how long it takes to prepare num_of_layers where

    :param num_of_layers: number of layers
    :return: preparation time in minutes

    Multiplies `num_of_layers` by `LAYER_PREPARATION_TIME` to and returns the result
    """
    return num_of_layers * LAYER_PREPARATION_TIME


def elapsed_time_in_minutes(num_of_layers: int, elapsed_bake_time: int) -> int:
    """Calculates how much time has passed

    :param num_of_layers:
    :param elapsed_bake_time:
    :return: elapsed time in minutes
    """
    return preparation_time_in_minutes(num_of_layers=num_of_layers) + elapsed_bake_time

class MechFighter:
    """A giant mech with a custom weapon"""

    weapon_name: str

    def __init__(self, weapon_name: str):
        """Create a mech fighter with a custom weapon name

        Args:
            weapon_name (str): Name of weapon
        """
        self.weapon_name = weapon_name

    def print_weapon_name(self):
        print(self.weapon_name)

    def change_weapon_name(self, new_name: str):
        self.weapon_name = new_name


def main():
    moonlight_wrecker: MechFighter = MechFighter(weapon_name="Rose sword")
    moonlight_wrecker.print_weapon_name()
    moonlight_wrecker.change_weapon_name(new_name="Black spear")
    moonlight_wrecker.print_weapon_name()
    # Rose sword
    # Black spear

if __name__ == "__main__":
    main()

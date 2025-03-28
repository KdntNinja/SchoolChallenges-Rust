fuel_prices = {
    1: 1.40,  # Petrol
    2: 1.55,  # Diesel
    3: 0.95   # LPG
}

fuel_type = int(input("(1) Petrol\n(2) Diesel\n(3) LPG\n: "))

print(f"Price per liter: £{fuel_prices.get(fuel_type, 'Invalid choice')}")

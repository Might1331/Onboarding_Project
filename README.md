# Onboarding_Project
TypeDB Data Model

### Dataset
The data used here was generated via dataGenerator.py script in the ./src/ directory. To get a new dataset run the script and it will write the data.tql file for you.

### Questions to explore
1. What places could buy raw_food=$x ?
2. Get count of non-vegetarian outlets with vegetarian specialities.
3. Get count raw items sold at places with avg_rating more tha $r and has a dish using it as raw_ingredient with price greater than $p units.

### Setup
To get this repository, run the following command inside your git enabled terminal
```bash
$ git clone https://github.com/Might1331/restaurant-menu-data-model.git
```

You will also need typedb server. Download the zip from  https://repo.vaticle.com/#browse/browse:artifact-snapshot:vaticle_typedb%2F07b9dfe04c786888a68f70b6f46dfdad1c9bb2e5%2Ftypedb-server-windows-07b9dfe04c786888a68f70b6f46dfdad1c9bb2e5.zip>.

**Note:: Click on the path to start the download.

Extract the files and run the following command within the "...\typedb-server-windows-07b.." directory
```
$ ./typedb server --server.address=localhost:1729
```

Once the server is running, head over to the root of this repo that you just cloned and run the rust program using the command
```
$ cargo run
```
The program should give you three query options to choose from:

```
Question 1:What places could buy raw_food=$x ?
Question 2:Get count of non-vegetarian outlets with vegetarian specialities.
Question 3:Get count raw items sold at places with avg_rating more tha $r and has a dish using it as raw_ingredient with price greater than $p units.
What query would you like to make? Enter 1,2 or 3.
```
Enter the parameters for the query.

### Example Query 1
#### TQL query
```
match 
$rf isa raw_food, has name "brocolli12";
$is_i (raw_food: $rf,dish: $d) isa is_ingredient;
$sl (seller: $m,product: $d) isa sells;
$m has name $n,has call_number $c;
get $n;
```

#### Answer
```
Q1->What places could buy raw_food=$x ?
Q2->Get count of non-vegetarian outlets with vegetarian specialities.
Q3->Get count raw items sold at places with avg_rating more tha $r and has a dish using it as raw_ingredient with price greater than $p units.
What query would you like to make? Enter 1,2 or 3.

1
Enter the name of raw_food x : 
suger42
::Q1::
All Burger
Elite's Non Vegas
All Non Vegas
Golden London
Indian Backery
Thai Food
All Hawaian
Hawaian Cooks
Brunch Garden
La Blue
London Delicacy
Veg Tour
Patty Diner
Japanese Food
Golden Japanese
London House
Old Burger Show
Grand Irish Eatarry
100 Non Vegas Show
Fast Irish Delicacy
The Unlimited
All Blue
Grand Irish
Old Japanese
La Arabian
Elite's Pizza Cafe
Burger Delicacy
London Backery
La Brunch
```

### Example Query 2
#### TQL query
```
match 
$m2 isa menu, has is_vegetarian false,has name $n2;
$d2 isa dish, has is_vegetarian false;
$sp2 (restaurant: $m2,$d2) isa speciality;
get $m2;count;
```

#### Answer

```
Q1->What places could buy raw_food=$x ?
Q2->Get count of non-vegetarian outlets with vegetarian specialities.
Q3->Get count raw items sold at places with avg_rating more tha $r and has a dish using it as raw_ingredient with price greater than $p units.
What query would you like to make? Enter 1,2 or 3.

2
::Q2::
Answe for Q2:  10
```
### Example Query 3

#### TQL query
```
match
$m3 isa menu, has avg_rating>r,has name $mn3;
$d3 isa dish, has name $dn3;
$sl3 (seller: $m3,product: $d3) isa sells, has price $p3;
$p3>p;
$rf3 isa raw_food,has name $rfn3;
$isn3 (raw_food: $rf3,$d3) isa is_ingredient;
get $rfn3;
```

#### Answer
```
Q1->What places could buy raw_food=$x ?
Q2->Get count of non-vegetarian outlets with vegetarian specialities.
Q3->Get count raw items sold at places with avg_rating more tha $r and has a dish using it as raw_ingredient with price greater than $p units.
What query would you like to make? Enter 1,2 or 3.

3
Enter the avg_rating of Restraunt r :
4.3
Enter the threshold price for raw_ingredient p :
49.5
::Q3::
chicken457
brocolli108
souy sauce115
tomato sauce285
cheese412
souy sauce19
pwarn243
brocolli204
fish24
red sauce428
spice206
beet roots119
tamarind sauce310
msg183
brocolli348
msg15
milk371
pwarn51
masala208
cabbage271
tamarind sauce478
masala376
milk299
rice490
pwarn99
beans61
cabbage343
masala400
spice302
```

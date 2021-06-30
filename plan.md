## Gonna use C# naming conventions because I don't know rust naming conventions

For various inputs you may use pointers in the parameters but I'm not going to put that in because i cba

### Transaction(User buyer, User seller, Item itemBeingSold, double pricePerItem, int quantity) 

Used for the exchange of currency for items; other functions should use this one whenever they perform a transaction

    Check buyer.balance >= pricePerItem * quantity;
    var stock = Find stock from itemBeingSold and seller inputs;
    Check stock >= quantity;
    buyer.balance -= pricePerItem * quantity;
    // something to award the buyer the items, i asked in #dev but you didn't respond stinky poopy
    end;

### AddTradeListing(User seller, Item itemBeingSold, int maximumIndividualPurchases, int stock, double pricePerItem)

I think a trade listing makes more sense than what I described as a permanent market, but the same idea, just a different name. This function will allow a seller to push a trade listing to the global board, where they can define: maximum purchases per buyer, stock to sell, price per item sold, item being sold

### RemoveTradeListing()

Reverse of `AddTradeListing`

### BuyTradeListing()

For buyers to actually buy from a trade listing

### Other cool stuff

- Recording average pricePerItem for new players to get a rough idea of price: maybe even fancy normal distribution graphs from this data
- 

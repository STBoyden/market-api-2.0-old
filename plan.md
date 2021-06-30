## Gonna use C# naming conventions because I don't know rust naming conventions

For various inputs you may use pointers in the parameters but I'm not going to put that in because i cba

### ItemTransaction(User buyer, User seller, Item itemBeingSold, double pricePerItem, int quantity) 

Used for the exchange of currency for items; other functions should use this one whenever they perform a transaction

    Check buyer.balance >= pricePerItem * quantity;
    var stock = Find stock from itemBeingSold and seller inputs;
    Check stock >= quantity;
    buyer.balance -= pricePerItem * quantity;
    // something to award the buyer the items, i asked in #dev but you didn't respond stinky poopy
    end;
    
### ServiceTransaction(User buyer, User seller, User checker, double serviceCost)

Used for the exchange of currency for services; `checker` is a third-party body who will confirm service has been completed or not. This is to avoid fraud

    Check buyer.balance >= serviceCost;
    Cache buyer.balance - buyer.balance -= serviceCost;
    Await seller to submit application for service completion
    Await checker to confirm service completion
    seller.balance += serviceCost;
    end;

### AddTradeListing(User seller, Item itemBeingSold, int maximumIndividualPurchases, int stock, double pricePerItem)
#### Overload for services

I think a trade listing makes more sense than what I described as a permanent market, but the same idea, just a different name. This function will allow a seller to push a trade listing to the global board, where they can define: maximum purchases per buyer, stock to sell, price per item sold, item being sold

### RemoveTradeListing()

Reverse of `AddTradeListing`

### BuyTradeListing()

For buyers to actually buy from a trade listing

## Other cool stuff

- Recording average pricePerItem for new players to get a rough idea of price: maybe even fancy normal distribution graphs from this data
- Allow sellers to sell shares of their business to people - for startup money 
- Rating system for buyers/sellers to show their integrity
- Loans (possibly should be in the bank? But the bank is more a centralised system which doesn't apply too well, right? The bank reserve is for initial currency population, it has no money for giving out loans)

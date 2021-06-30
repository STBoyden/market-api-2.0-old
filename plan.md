## Gonna use C# naming conventions because I don't know rust naming conventions

For various inputs you may use pointers in the parameters but I'm not going to put that in because i cba

### Transaction(User buyer, User seller, Item itemType, double pricePerItem, int quantity) 

    Check buyer.balance >= pricePerItem * quantity;
    var stock = Find stock from itemType and seller inputs;
    Check stock >= quantity;
    buyer.balance -= pricePerItem * quantity;
    // something to award the buyer the items, i asked in #dev but you didn't respond stinky poopy
    end;



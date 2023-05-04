diesel::table!(
    pub users (uuid){
        uuid->Text,
        username->Text,
        password->Text,
        create_at->Text,
    }
);

diesel::table! {
    pub customers(id){
        id->Integer,
        parent_uuid->Text,
        customer_name->Text,
        phone->Nullable<Text>,
        total_debts->Nullable<Integer>, 
    }
}

diesel::table! {
    pub orders(id){
        id->Integer,
        customer_id->Integer,
        amount->Integer,
        status->Bool,
        create_at->Text,   
        remark->Nullable<Text>,
    }
}
.load dist/debug/fake0
.mode box
.header on

select fake();

select fake_credit_card();

select fake_name();

select fake_phone_number();

select fake_file_extension();
select fake_mimetype();

--select name from pragma_function_list order by name;

select fake_rgba_color(), fake_hex_color();

select fake_bs();

select fake_rfc_status_code();
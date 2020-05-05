# rid

Generates resource identifiers `rid` with contextual parts. Inspired by Amazon Resource Names (ARN).


**WIP - Repo is brand new**

## Templates

Default template for creating a resource id `rid` is:

```
rid:partition:service:region:account-id:resource-id
rid:partition:service:region:account-id:resource-type/resource-id
rid:partition:service:region:account-id:resource-type:resource-id
```

Templates can be customized.


## `rid` examples

Let's create `rid`s for emails for `homer` with user id `1234` on various `example.com` sub domains globally.

```	
rid:app:email:us:1234:address/homer@example.com
rid:app:email:us:1234:address/homer-work@example.com
```

Let's give `rid`s to some web servers for `example.com` domain in US and Europe.

```
rid:app:web:us:1234:server/web01.us.example.com
rid:app:web:eu:1234:server/web01.eu.example.com
```

And `rid` of a web user account.

```
rid:app:iam:global:1234:uid/1234
```


## API

todo


## Requirements

todo


## License

to be selected...













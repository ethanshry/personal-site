import * as cdk from '@aws-cdk/core';
import * as s3 from '@aws-cdk/aws-s3';
import * as lambda from '@aws-cdk/aws-lambda';
import * as s3deploy from '@aws-cdk/aws-s3-deployment';

export class CdkStack extends cdk.Stack {
  constructor(scope: cdk.Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const bucket = new s3.Bucket(this, 'MyFirstBucket', {
      versioned: true,
      bucketName: "testBucket",
      websiteIndexDocument: 'index.html',
      publicReadAccess: true
    });

    new s3deploy.BucketDeployment(this, "BucketDeployment", {
      sources: [s3deploy.Source.asset('../../site/public')],
      destinationBucket(bucket),
      retainOnDelete: false
    })

    const api = new lambda.Function(this, "Backend", {
      runtime: lambda.Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset('../../blog_api/lambda.zip'),
      handler: 'func'
    })
  }
}
